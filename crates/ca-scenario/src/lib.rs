use ca_identity::RegionId;
use ca_snapshot::FleetSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scenario {
    pub id: String,
    pub weight: f64,
    pub region_capacity: BTreeMap<RegionId, f64>,
    pub latency_multiplier: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScenarioSet(pub Vec<Scenario>);
impl ScenarioSet {
    pub fn frozen(snapshot: &FleetSnapshot, failed: &RegionId) -> Self {
        let factors = [
            ("nominal", 0.55, 1.0, 3.0),
            ("correlated", 0.25, 0.55, 4.2),
            ("recovery-lag", 0.20, 0.35, 5.0),
        ];
        Self(
            factors
                .into_iter()
                .map(|(id, weight, failed_factor, latency)| {
                    let mut m = snapshot
                        .regions
                        .keys()
                        .map(|r| (r.clone(), 1.0))
                        .collect::<BTreeMap<_, _>>();
                    m.insert(failed.clone(), failed_factor);
                    Scenario {
                        id: id.into(),
                        weight,
                        region_capacity: m,
                        latency_multiplier: latency,
                    }
                })
                .collect(),
        )
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_type() {
        assert!(ScenarioSet(vec![]).is_empty());
    }
}
