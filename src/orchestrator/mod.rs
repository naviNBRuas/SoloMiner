use crate::core::miner::{start_mining_instance, Sha256Miner, RandomXMiner};
use crate::core::DeviceType;
use crate::telemetry::MinerMetrics;
use std::sync::Arc;
use tokio::sync::Mutex;
use clap::ValueEnum;
use tracing::{info, warn};

#[derive(Debug, Clone, ValueEnum)]
pub enum MiningMode {
    Performance,
    Conservative,
}

pub fn get_recommended_threads(mode: &MiningMode) -> usize {
    let num_cpus = num_cpus::get();
    match mode {
        MiningMode::Performance => num_cpus,
        MiningMode::Conservative => (num_cpus / 2).max(1),
    }
}

pub async fn start_orchestrator(
    mode: MiningMode,
    wallet_address: String,
    difficulty: String,
    metrics: Arc<MinerMetrics>,
    stop_signal: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let num_threads = get_recommended_threads(&mode);
    let mut handles = vec![];
    
    info!(mode = ?mode, threads = num_threads, difficulty = %difficulty, "Starting mining orchestrator");

    // CPU SHA-256 Miners
    for i in 0..num_threads {
        let algo = Arc::new(Sha256Miner::new(DeviceType::CPU));
        let m = metrics.clone();
        let wallet = wallet_address.clone();
        let diff = difficulty.clone();
        let stop = stop_signal.clone();
        
        handles.push(tokio::spawn(async move {
            if let Err(e) = start_mining_instance(i, wallet, algo, m, diff, stop).await {
                warn!(instance = i, error = %e, "CPU miner instance error");
            }
        }));
    }

    // CPU RandomX Miners (half the CPU threads)
    let randomx_threads = (num_threads / 2).max(1);
    for i in 0..randomx_threads {
        let algo = Arc::new(RandomXMiner::new(DeviceType::CPU));
        let m = metrics.clone();
        let wallet = wallet_address.clone();
        let diff = difficulty.clone();
        let stop = stop_signal.clone();
        let instance_id = num_threads + i;
        
        handles.push(tokio::spawn(async move {
            if let Err(e) = start_mining_instance(instance_id, wallet, algo, m, diff, stop).await {
                warn!(instance = instance_id, error = %e, "RandomX miner instance error");
            }
        }));
    }

    // GPU Miners (Simulated)
    {
        // SHA-256 GPU Miner
        let algo = Arc::new(Sha256Miner::new(DeviceType::GPU));
        let m = metrics.clone();
        let wallet = wallet_address.clone();
        let diff = difficulty.clone();
        let stop = stop_signal.clone();
        handles.push(tokio::spawn(async move {
            if let Err(e) = start_mining_instance(99, wallet, algo, m, diff, stop).await {
                warn!(instance = 99, error = %e, "GPU SHA-256 miner error");
            }
        }));
    }
    
    {
        // RandomX GPU Miner
        let algo = Arc::new(RandomXMiner::new(DeviceType::GPU));
        let m = metrics.clone();
        let wallet = wallet_address.clone();
        let diff = difficulty.clone();
        let stop = stop_signal.clone();
        handles.push(tokio::spawn(async move {
            if let Err(e) = start_mining_instance(100, wallet, algo, m, diff, stop).await {
                warn!(instance = 100, error = %e, "GPU RandomX miner error");
            }
        }));
    }

    info!(total_instances = handles.len(), "All mining instances started successfully");
    
    // We don't join all because we want to run the TUI or dashboard in parallel
    // The handles are kept alive as long as the main process runs or until stop_signal is set
    
    Ok(())
}
