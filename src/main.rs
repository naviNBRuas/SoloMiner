use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use solo_miner::{config, orchestrator, telemetry, tui};
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(
    name = "_SoloMiner_",
    author = "The Lonely Miner",
    version = "1.0.0",
    about = "⛏️ Lonely Solo Miner - Mining in magnificent isolation",
    long_about = "Lonely Solo Miner is the ultimate tool for the fiercely independent crypto miner. \
Experience the thrill of solo mining with beautiful terminal visuals, real-time analytics, \
and the satisfaction of knowing you're doing it all by yourself."
)]
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
    /// Starts the miner with beautiful terminal interface
    Start {
        /// Mining mode: performance (all cores) or conservative (half cores)
        #[arg(long, default_value = "performance")]
        mode: orchestrator::MiningMode,
        /// Mining algorithm: sha256 or randomx
        #[arg(long, default_value = "sha256")]
        algorithm: AlgorithmType,
        /// Run in background without TUI
        #[arg(long, default_value = "false")]
        no_tui: bool,
        /// Custom difficulty setting
        #[arg(long, default_value = "0000")]
        difficulty: Option<String>,
    },
    /// Starts only the web dashboard
    Dashboard {
        /// Port for web dashboard (default: 8080)
        #[arg(long, default_value = "8080")]
        port: u16,
    },
    /// Shows current mining statistics
    Status,
    /// Stops all mining processes
    Stop,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Install error handlers
    color_eyre::install()?;

    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer().compact())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();
    
    // Load configuration
    let config = config::Config::load()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to load config: {}", e))?
        .unwrap_or_else(|| {
            println!("🔧 Using default configuration");
            config::Config::from_env()
        });
    
    // Validate configuration
    config.validate()
        .map_err(|e| color_eyre::eyre::eyre!("Configuration validation failed: {}", e))?;
    
    println!("⚙️  Configuration loaded:");
    println!("   Difficulty: {}", config.miner.difficulty);
    println!("   Threads: {}", config.miner.threads);
    println!("   Algorithm: {}", config.miner.algorithm);
    println!("   Telemetry Port: {}", config.telemetry.port);
    
    // Initialize shared metrics
    let metrics = Arc::new(telemetry::MinerMetrics::new());
    let stop_signal = Arc::new(Mutex::new(false));

    match &cli.command {
        Commands::Start { mode, algorithm: _, no_tui, difficulty } => {
            let wallet_address = env::var("WALLET_ADDRESS")
                .unwrap_or_else(|_| "L0N3LY-W4LL3T-4DDR355".to_string());
            
            let actual_difficulty = difficulty.clone()
                .unwrap_or(config.miner.difficulty.clone());
            
            // Store clones for printing later
            let wallet_for_print = wallet_address.clone();
            let difficulty_for_print = actual_difficulty.clone();
            
            // Update status
            {
                let mut status = metrics.status.write().await;
                *status = "Mining".to_string();
            }

            // Start mining orchestrator
            orchestrator::start_orchestrator(
                mode.clone(),
                wallet_address,
                actual_difficulty,
                metrics.clone(),
                stop_signal.clone(),
            ).await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to start orchestrator: {}", e))?
            ;

            if *no_tui {
                println!("\n⛏️  Lonely Solo Miner started in background mode");
                println!("Wallet: {}", wallet_for_print);
                println!("Difficulty: {}", difficulty_for_print);
                println!("Press Ctrl+C to stop mining\n");
                
                // Keep main alive
                tokio::signal::ctrl_c().await?;
                *stop_signal.lock().await = true;
                println!("\n🛑 Mining stopped");
            } else {
                // Run beautiful TUI
                if let Err(e) = tui::run_tui(metrics.clone()).await {
                    eprintln!("TUI error: {}", e);
                }
                *stop_signal.lock().await = true;
            }
        }
        Commands::Dashboard { port } => {
            telemetry::start_dashboard(metrics.clone(), *port).await?;
        }
        Commands::Status => {
            let snapshot = metrics.snapshot().await;
            println!("\n⛏️  Lonely Solo Miner Status");
            println!("=========================");
            println!("Status: {}", snapshot.status);
            println!("Hashrate: {} H/s", snapshot.hashrate);
            println!("Total Hashes: {}", snapshot.total_hashes);
            println!("Blocks Found: {}", snapshot.blocks_found);
            println!("Uptime: {} seconds\n", snapshot.uptime);
        }
        Commands::Stop => {
            println!("\n🛑 Stopping all mining processes...");
            *stop_signal.lock().await = true;
            println!("✅ All processes stopped\n");
        }
    }
    
    Ok(())
}