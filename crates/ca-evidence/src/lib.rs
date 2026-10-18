use ca_events::DomainEvent;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::Path,
};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub count: usize,
    pub root: String,
}
#[derive(Debug, Error)]
pub enum EvidenceError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}
pub fn append(path: &Path, events: &[DomainEvent]) -> Result<Receipt, EvidenceError> {
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    let mut h = Sha256::new();
    for e in events {
        let line = serde_json::to_vec(e)?;
        h.update(&line);
        f.write_all(&line)?;
        f.write_all(b"\n")?;
    }
    Ok(Receipt {
        count: events.len(),
        root: format!("{:x}", h.finalize()),
    })
}
pub fn readback(path: &Path) -> Result<Vec<DomainEvent>, EvidenceError> {
    let f = OpenOptions::new().read(true).open(path)?;
    BufReader::new(f)
        .lines()
        .map(|l| Ok(serde_json::from_str(&l?)?))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_roundtrip() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("j");
        append(&p, &[]).unwrap();
        assert!(readback(&p).unwrap().is_empty());
    }
}
