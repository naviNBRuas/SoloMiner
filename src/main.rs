use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

mod config;
mod core;
mod orchestrator;
mod telemetry;

#[derive(Error, Debug)]
enum SoloMinerError {
    #[error("Environment variable not set: {0}")]
    EnvVarError(#[from] std::env::VarError),
    #[error("Failed to start dashboard: {0}")]
    DashboardError(#[from] std::io::Error),
    #[error("Mining failed: {0}")]
    MiningError(String),
    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

#[derive(Parser)]
#[command(name = "SoloMiner")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "A professional, high-performance, cross-platform SoloMiner.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum AlgorithmType {
    Sha256,
    RandomX,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Starts the miner
    Start { 
        #[arg(long, default_value = "conservative")]
        mode: orchestrator::MiningMode,
        #[arg(long, default_value = "sha256")]
        algorithm: AlgorithmType,
    },
    /// Stops the miner
    Stop,
    /// Shows the status of the miner
    Status,
    /// Starts the web dashboard
    Dashboard,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    let config = config::Config::load()?;
    let metrics = Arc::new(Mutex::new(telemetry::MinerMetrics::default()));

    match &cli.command {
        Commands::Start { mode, algorithm } => {
            println!("Starting miner in {:?} mode...", mode);
            let wallet_address = env::var("WALLET_ADDRESS")?;
            let num_threads = orchestrator::get_recommended_threads(mode.clone());
            println!("Using {} mining threads.", num_threads);

            let selected_algorithm: Box<dyn core::MinerAlgorithm> = match algorithm {
                AlgorithmType::Sha256 => Box::new(core::miner::Sha256Miner),
                AlgorithmType::RandomX => Box::new(core::miner::RandomXMiner),
            };

            core::miner::start_mining(&wallet_address, num_threads, selected_algorithm, metrics.clone(), &config.miner.difficulty).await?;
        }
        Commands::Stop => {
            println!("Stopping miner...");
        }
        Commands::Status => {
            println!("Miner status: Idle");
        }
        Commands::Dashboard => {
            telemetry::start_dashboard(metrics.clone()).await?;
        }
    }
    Ok(())
}