use ca_identity::ServiceId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventKind {
    SnapshotFrozen,
    ScenariosGenerated,
    AnalogueRecalled,
    PlanRanked,
    PolicyEvaluated,
    EvidenceSealed,
    RecoveryVerified,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainEvent {
    pub sequence: u64,
    pub kind: EventKind,
    pub subject: ServiceId,
    pub detail: String,
}
impl DomainEvent {
    pub fn new(
        sequence: u64,
        kind: EventKind,
        subject: ServiceId,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            sequence,
            kind,
            subject,
            detail: detail.into(),
        }
    }
}
