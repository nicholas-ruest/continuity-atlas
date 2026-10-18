use ca_identity::{RegionId, ServiceId};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyInput {
    pub plan_id: String,
    pub moves: Vec<(ServiceId, RegionId)>,
    pub tier0_violation: bool,
    pub destination_utilization: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub allowed: bool,
    pub engine: String,
    pub reason: String,
}
