use thiserror::Error;
#[derive(Clone, Debug)]
pub struct Config {
    pub alpha: f64,
    pub wasserstein_radius: f64,
    pub recall_k: usize,
}
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid alpha, radius or recall k")]
    Invalid,
}
impl Config {
    pub fn validate(self) -> Result<Self, ConfigError> {
        if (0.0..1.0).contains(&self.alpha)
            && self.wasserstein_radius >= 0.0
            && self.wasserstein_radius <= 1.0
            && self.recall_k > 0
        {
            Ok(self)
        } else {
            Err(ConfigError::Invalid)
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            alpha: 0.67,
            wasserstein_radius: 0.15,
            recall_k: 3,
        }
    }
}
