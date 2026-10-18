use ca_events::DomainEvent;
use tracing::info;
pub fn init() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_test_writer()
        .try_init();
}
pub fn record(e: &DomainEvent) {
    info!(sequence=e.sequence,kind=?e.kind,subject=%e.subject.as_str(),"continuity_event");
}
