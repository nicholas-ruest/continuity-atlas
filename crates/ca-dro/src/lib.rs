use ca_loss::LossVector;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RobustScore {
    pub empirical_mean: f64,
    pub cvar: f64,
    pub ambiguity_penalty: f64,
    pub robust_upper: f64,
}
#[derive(Clone, Debug)]
pub struct RobustRanker {
    alpha: f64,
    radius: f64,
}
#[derive(Debug, Error)]
pub enum DroError {
    #[error("losses, alpha or radius invalid")]
    Invalid,
}
impl RobustRanker {
    pub fn new(alpha: f64, radius: f64) -> Result<Self, DroError> {
        if !(0.0..1.0).contains(&alpha) || radius < 0.0 || !radius.is_finite() {
            return Err(DroError::Invalid);
        }
        Ok(Self { alpha, radius })
    }
    pub fn score(&self, losses: &LossVector) -> Result<RobustScore, DroError> {
        if losses.0.is_empty() || losses.0.iter().any(|x| !x.is_finite() || *x < 0.0) {
            return Err(DroError::Invalid);
        }
        let mut v = losses.0.clone();
        v.sort_by(f64::total_cmp);
        let empirical_mean = v.iter().sum::<f64>() / v.len() as f64;
        let start = ((v.len() as f64 * self.alpha).floor() as usize).min(v.len() - 1);
        let cvar = v[start..].iter().sum::<f64>() / (v.len() - start) as f64;
        let span = v.last().unwrap() - v.first().unwrap();
        let ambiguity_penalty = self.radius * (1.0 + span);
        Ok(RobustScore {
            empirical_mean,
            cvar,
            ambiguity_penalty,
            robust_upper: cvar + ambiguity_penalty,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn radius_monotone() {
        let l = LossVector(vec![1., 2., 8.]);
        let a = RobustRanker::new(0.67, 0.0).unwrap().score(&l).unwrap();
        let b = RobustRanker::new(0.67, 0.2).unwrap().score(&l).unwrap();
        assert!(b.robust_upper > a.robust_upper);
    }
}
