use serde::{Deserialize, Serialize};
use std::{fs, env};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Configuration validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MinerConfig {
    #[serde(default = "default_difficulty")]
    pub difficulty: String,
    #[serde(default = "default_threads")]
    pub threads: usize,
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

fn default_difficulty() -> String { "0000".to_string() }
fn default_threads() -> usize { num_cpus::get() }
fn default_algorithm() -> String { "sha256".to_string() }
fn default_batch_size() -> usize { 10000 }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_format")]
    pub format: String,
}

fn default_log_level() -> String { "info".to_string() }
fn default_log_format() -> String { "compact".to_string() }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelemetryConfig {
    #[serde(default = "default_telemetry_port")]
    pub port: u16,
    #[serde(default = "default_enable_metrics")]
    pub enable_metrics: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub miner: MinerConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub telemetry: TelemetryConfig,
}

fn default_telemetry_port() -> u16 { 8080 }
fn default_enable_metrics() -> bool { true }

impl Default for MinerConfig {
    fn default() -> Self {
        Self {
            difficulty: default_difficulty(),
            threads: default_threads(),
            algorithm: default_algorithm(),
            batch_size: default_batch_size(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
        }
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            port: default_telemetry_port(),
            enable_metrics: default_enable_metrics(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            miner: MinerConfig::default(),
            logging: LoggingConfig::default(),
            telemetry: TelemetryConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Option<Self>, ConfigError> {
        match fs::read_to_string("config.toml") {
            Ok(config_str) => {
                debug!("Loading configuration from config.toml");
                let config: Config = toml::from_str(&config_str)?;
                info!("Configuration loaded successfully");
                Ok(Some(config))
            },
            Err(e) => {
                warn!("Could not read config.toml: {}", e);
                warn!("Using default configuration");
                Ok(None)
            }
        }
    }
    
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate difficulty format
        if self.miner.difficulty.is_empty() || self.miner.difficulty.len() > 16 {
            return Err(ConfigError::Validation("Invalid difficulty setting".to_string()));
        }
        
        // Validate threads (0 means auto-detect, which is valid)
        if self.miner.threads > 128 {
            return Err(ConfigError::Validation("Invalid thread count".to_string()));
        }
        
        // Validate algorithm
        let valid_algorithms = ["sha256", "randomx"];
        if !valid_algorithms.contains(&self.miner.algorithm.as_str()) {
            return Err(ConfigError::Validation("Invalid algorithm specified".to_string()));
        }
        
        // Validate batch size
        if self.miner.batch_size < 100 || self.miner.batch_size > 1_000_000 {
            return Err(ConfigError::Validation("Invalid batch size".to_string()));
        }
        
        // Validate telemetry port
        if self.telemetry.port < 1024 {
            return Err(ConfigError::Validation("Invalid telemetry port".to_string()));
        }
        
        Ok(())
    }
    
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        // Override with environment variables
        if let Ok(difficulty) = env::var("MINER_DIFFICULTY") {
            config.miner.difficulty = difficulty;
        }
        
        if let Ok(threads) = env::var("MINER_THREADS") {
            if let Ok(num) = threads.parse::<usize>() {
                config.miner.threads = num;
            }
        }
        
        if let Ok(algorithm) = env::var("MINER_ALGORITHM") {
            config.miner.algorithm = algorithm.to_lowercase();
        }
        
        if let Ok(port) = env::var("TELEMETRY_PORT") {
            if let Ok(num) = port.parse::<u16>() {
                config.telemetry.port = num;
            }
        }
        
        config
    }
}
