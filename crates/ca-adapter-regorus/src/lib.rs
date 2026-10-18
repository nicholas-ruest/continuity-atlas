use ca_policy::{PolicyDecision, PolicyInput};
use regorus::{Engine, Value};
use thiserror::Error;

const POLICY: &str = r#"package continuity
default allow := false
allow if { not input.tier0_violation; input.destination_utilization <= 0.85 }
"#;

#[derive(Debug, Error)]
pub enum RegorusError {
    #[error("rego: {0}")]
    Engine(String),
    #[error("rego returned non-boolean")]
    NonBoolean,
}

#[derive(Default)]
pub struct RegorusPolicy;

impl RegorusPolicy {
    pub fn evaluate(&self, input: &PolicyInput) -> Result<PolicyDecision, RegorusError> {
        let mut engine = Engine::new();
        engine
            .add_policy("continuity.rego".into(), POLICY.into())
            .map_err(|error| RegorusError::Engine(error.to_string()))?;
        engine
            .add_data_json(r#"{"max_utilization":0.85}"#)
            .map_err(|error| RegorusError::Engine(error.to_string()))?;
        engine
            .set_input_json(
                &serde_json::to_string(input)
                    .map_err(|error| RegorusError::Engine(error.to_string()))?,
            )
            .map_err(|error| RegorusError::Engine(error.to_string()))?;
        let value = engine
            .eval_rule("data.continuity.allow".into())
            .map_err(|error| RegorusError::Engine(error.to_string()))?;
        let allowed = match value {
            Value::Bool(value) => value,
            _ => return Err(RegorusError::NonBoolean),
        };
        Ok(PolicyDecision {
            allowed,
            engine: "microsoft-regorus-0.13.0".into(),
            reason: if allowed {
                "continuity invariant satisfied"
            } else {
                "tier-0 or utilization violation"
            }
            .into(),
        })
    }
}

impl ca_ports::PolicyEngine for RegorusPolicy {
    type Error = RegorusError;

    fn evaluate(&self, input: &PolicyInput) -> Result<PolicyDecision, Self::Error> {
        RegorusPolicy::evaluate(self, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_allow_deny() {
        let policy = RegorusPolicy;
        let mut input = PolicyInput {
            plan_id: "safe".into(),
            moves: vec![],
            tier0_violation: false,
            destination_utilization: 0.8,
        };
        assert!(policy.evaluate(&input).unwrap().allowed);
        input.tier0_violation = true;
        assert!(!policy.evaluate(&input).unwrap().allowed);
    }
}
