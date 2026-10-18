use ca_events::DomainEvent;
use ca_policy::{PolicyDecision, PolicyInput};
pub trait PolicyEngine {
    type Error: std::error::Error + Send + Sync + 'static;
    fn evaluate(&self, input: &PolicyInput) -> Result<PolicyDecision, Self::Error>;
}
pub trait IncidentMemory {
    type Error: std::error::Error + Send + Sync + 'static;
    fn remember(
        &self,
        id: &str,
        event: &DomainEvent,
        vector: Vec<f32>,
    ) -> Result<String, Self::Error>;
    fn similar(&self, vector: Vec<f32>, k: usize) -> Result<usize, Self::Error>;
}
pub trait EvidenceSealer {
    type Error: std::error::Error + Send + Sync + 'static;
    fn seal(&self, events: &[DomainEvent]) -> Result<(), Self::Error>;
}
