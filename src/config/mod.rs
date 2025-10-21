use serde::Deserialize;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Configuration not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Deserialize)]
pub struct MinerConfig {
    pub difficulty: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    #[allow(dead_code)]
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub miner: MinerConfig,
    #[allow(dead_code)]
    pub logging: LoggingConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_str = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
