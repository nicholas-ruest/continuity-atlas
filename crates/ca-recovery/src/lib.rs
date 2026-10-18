use ca_evidence::Receipt;
use ca_plan::FailoverPlan;
use ca_service::Service;
use thiserror::Error;
#[derive(Clone, Debug)]
pub struct RecoveryProof {
    pub tier0_preserved: bool,
    pub receipt_bound: bool,
    pub plan_id: String,
}
#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("recovery violates continuity or evidence binding")]
    Invalid,
}
pub fn replay(
    plan: &FailoverPlan,
    services: &[Service],
    receipt: &Receipt,
) -> Result<RecoveryProof, RecoveryError> {
    let tier0_preserved = !plan.tier0_violation
        && services
            .iter()
            .filter(|s| s.tier == 0)
            .all(|s| s.min_healthy_regions >= 2);
    let receipt_bound = receipt.count > 0 && receipt.root.len() == 64;
    if tier0_preserved && receipt_bound {
        Ok(RecoveryProof {
            tier0_preserved,
            receipt_bound,
            plan_id: plan.id.clone(),
        })
    } else {
        Err(RecoveryError::Invalid)
    }
}
