use ca_identity::{RegionId, ServiceId};
use ca_region::Region;
use ca_service::Service;
use ca_topology::Topology;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Placement {
    pub service: ServiceId,
    pub region: RegionId,
    pub capacity: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FleetSnapshot {
    pub observed_at: u64,
    pub regions: BTreeMap<RegionId, Region>,
    pub services: BTreeMap<ServiceId, Service>,
    pub placements: Vec<Placement>,
}
#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("snapshot is stale, empty or references unknown objects")]
    Invalid,
}
impl FleetSnapshot {
    pub fn build(
        observed_at: u64,
        topology: &Topology,
        regions: Vec<Region>,
        placements: Vec<Placement>,
    ) -> Result<Self, SnapshotError> {
        let services = topology
            .services()
            .map(|s| (s.id.clone(), s.clone()))
            .collect::<BTreeMap<_, _>>();
        let regions = regions
            .into_iter()
            .map(|r| (r.id.clone(), r))
            .collect::<BTreeMap<_, _>>();
        if observed_at == 0
            || services.is_empty()
            || regions.is_empty()
            || placements.iter().any(|p| {
                p.capacity == 0
                    || !services.contains_key(&p.service)
                    || !regions.contains_key(&p.region)
            })
        {
            return Err(SnapshotError::Invalid);
        }
        Ok(Self {
            observed_at,
            regions,
            services,
            placements,
        })
    }
    pub fn capacity(&self, s: &ServiceId, r: &RegionId) -> u32 {
        self.placements
            .iter()
            .filter(|p| &p.service == s && &p.region == r)
            .map(|p| p.capacity)
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_fails() {
        assert!(FleetSnapshot::build(1, &Topology::default(), vec![], vec![]).is_err());
    }
}
