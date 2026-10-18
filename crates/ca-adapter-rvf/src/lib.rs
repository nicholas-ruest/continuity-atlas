use ca_events::DomainEvent;
use rvf_crypto::{WitnessEntry, create_witness_chain, shake256_256, verify_witness_chain};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Seal {
    pub chain: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum SealError {
    #[error("empty events or invalid witness")]
    Invalid,
}

pub fn seal(events: &[DomainEvent]) -> Result<Seal, SealError> {
    if events.is_empty() {
        return Err(SealError::Invalid);
    }
    let entries = events
        .iter()
        .map(|event| WitnessEntry {
            prev_hash: [0; 32],
            action_hash: shake256_256(
                format!("{}:{:?}:{}", event.sequence, event.kind, event.detail).as_bytes(),
            ),
            timestamp_ns: event.sequence,
            witness_type: 7,
        })
        .collect::<Vec<_>>();
    Ok(Seal {
        chain: create_witness_chain(&entries),
    })
}

pub fn verify(seal: &Seal) -> Result<(), SealError> {
    verify_witness_chain(&seal.chain)
        .map(|_| ())
        .map_err(|_| SealError::Invalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_events::EventKind;
    use ca_identity::ServiceId;

    #[test]
    fn tamper_rejected() {
        let event = DomainEvent::new(
            1,
            EventKind::EvidenceSealed,
            ServiceId::parse("x").unwrap(),
            "x",
        );
        let mut seal = seal(&[event]).unwrap();
        verify(&seal).unwrap();
        seal.chain[0] ^= 1;
        assert!(verify(&seal).is_err());
    }
}
