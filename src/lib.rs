use thiserror::Error;
use color_eyre::eyre::Report;

pub mod config;
pub mod core;
pub mod orchestrator;
pub mod telemetry;
pub mod tui;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SoloMinerError {
    #[error("Environment variable not set: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Failed to start dashboard: {0}")]
    DashboardError(#[from] std::io::Error),
    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Mining error: {0}")]
    MiningError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Generic error: {0}")]
    GenericError(#[from] Report),
}
