use ca_identity::ServiceId;
use ca_service::Service;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Topology {
    services: BTreeMap<ServiceId, Service>,
    edges: BTreeMap<ServiceId, BTreeSet<ServiceId>>,
}
#[derive(Debug, Error)]
pub enum TopologyError {
    #[error("duplicate service, missing endpoint, self edge or cycle")]
    Invalid,
}
impl Topology {
    pub fn add_service(&mut self, s: Service) -> Result<(), TopologyError> {
        if self.services.insert(s.id.clone(), s).is_some() {
            Err(TopologyError::Invalid)
        } else {
            Ok(())
        }
    }
    pub fn add_edge(&mut self, from: &ServiceId, to: &ServiceId) -> Result<(), TopologyError> {
        if from == to || !self.services.contains_key(from) || !self.services.contains_key(to) {
            return Err(TopologyError::Invalid);
        }
        self.edges
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
        if self.closure(to).contains(from) {
            self.edges.get_mut(from).unwrap().remove(to);
            return Err(TopologyError::Invalid);
        }
        Ok(())
    }
    pub fn closure(&self, root: &ServiceId) -> BTreeSet<ServiceId> {
        let mut seen = BTreeSet::new();
        let mut q = vec![root.clone()];
        while let Some(x) = q.pop() {
            if let Some(next) = self.edges.get(&x) {
                for n in next {
                    if seen.insert(n.clone()) {
                        q.push(n.clone());
                    }
                }
            }
        }
        seen
    }
    pub fn services(&self) -> impl Iterator<Item = &Service> {
        self.services.values()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cycle() {
        let mut t = Topology::default();
        let a = Service::new(ServiceId::parse("a").unwrap(), 1, 1, 1, 1).unwrap();
        let b = Service::new(ServiceId::parse("b").unwrap(), 1, 1, 1, 1).unwrap();
        t.add_service(a.clone()).unwrap();
        t.add_service(b.clone()).unwrap();
        t.add_edge(&a.id, &b.id).unwrap();
        assert!(t.add_edge(&b.id, &a.id).is_err());
    }
}
