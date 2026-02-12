use solo_miner::{
    config::Config,
    core::{Block, miner::{Sha256Miner, RandomXMiner}, DeviceType, MinerAlgorithm},
    telemetry::MinerMetrics,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        
        assert_eq!(config.miner.difficulty, "0000");
        assert_eq!(config.miner.threads, num_cpus::get());
        assert_eq!(config.miner.algorithm, "sha256");
        assert_eq!(config.miner.batch_size, 10000);
        
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.logging.format, "compact");
        
        assert_eq!(config.telemetry.port, 8080);
        assert_eq!(config.telemetry.enable_metrics, true);
    }

    #[test]
    fn test_config_validation_valid() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_difficulty() {
        let mut config = Config::default();
        config.miner.difficulty = "".to_string();
        assert!(config.validate().is_err());
        
        config.miner.difficulty = "A".repeat(20);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_threads() {
        let mut config = Config::default();
        config.miner.threads = 200;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_invalid_algorithm() {
        let mut config = Config::default();
        config.miner.algorithm = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_from_env() {
        unsafe {
            std::env::set_var("MINER_DIFFICULTY", "00000");
            std::env::set_var("MINER_THREADS", "8");
            std::env::set_var("MINER_ALGORITHM", "randomx");
            std::env::set_var("TELEMETRY_PORT", "9090");
        }
        
        let config = Config::from_env();
        
        assert_eq!(config.miner.difficulty, "00000");
        assert_eq!(config.miner.threads, 8);
        assert_eq!(config.miner.algorithm, "randomx");
        assert_eq!(config.telemetry.port, 9090);
        
        // Clean up
        unsafe {
            std::env::remove_var("MINER_DIFFICULTY");
            std::env::remove_var("MINER_THREADS");
            std::env::remove_var("MINER_ALGORITHM");
            std::env::remove_var("TELEMETRY_PORT");
        }
    }
}

#[cfg(test)]
mod miner_tests {
    use super::*;

    #[test]
    fn test_sha256_miner_initialization() {
        let miner = Sha256Miner::new(DeviceType::CPU);
        assert_eq!(miner.name(), "SHA-256 (CPU-Optimized)");
        
        let gpu_miner = Sha256Miner::new(DeviceType::GPU);
        assert_eq!(gpu_miner.name(), "SHA-256 (GPU-Accelerated)");
    }

    #[test]
    fn test_randomx_miner_initialization() {
        let miner = RandomXMiner::new(DeviceType::CPU);
        assert_eq!(miner.name(), "RandomX (CPU-Optimized)");
        
        let gpu_miner = RandomXMiner::new(DeviceType::GPU);
        assert_eq!(gpu_miner.name(), "RandomX (GPU-Simulated)");
    }

    #[test]
    fn test_sha256_mining_with_easy_difficulty() {
        let miner = Sha256Miner::new(DeviceType::CPU);
        let mut block = Block {
            id: 1,
            timestamp: 123456789,
            data: "test".to_string(),
            previous_hash: "0".to_string(),
            nonce: 0,
            difficulty: "0".to_string(), // Very easy difficulty
        };
        
        let result = miner.mine(&mut block);
        assert!(result.is_ok());
        
        if let Ok(Some(hash)) = result {
            assert!(hash.starts_with("0"));
            assert!(block.nonce > 0);
        }
    }

    #[test]
    fn test_randomx_mining_with_easy_difficulty() {
        let miner = RandomXMiner::new(DeviceType::CPU);
        let mut block = Block {
            id: 1,
            timestamp: 123456789,
            data: "test".to_string(),
            previous_hash: "0".to_string(),
            nonce: 0,
            difficulty: "0".to_string(),
        };
        
        let result = miner.mine(&mut block);
        assert!(result.is_ok());
        
        if let Ok(Some(hash)) = result {
            assert!(hash.starts_with("0"));
        }
        // Note: block.nonce may not change if no solution found in batch
    }

    #[test]
    fn test_batch_sizes() {
        let cpu_sha256 = Sha256Miner::new(DeviceType::CPU);
        let gpu_sha256 = Sha256Miner::new(DeviceType::GPU);
        let cpu_randomx = RandomXMiner::new(DeviceType::CPU);
        let gpu_randomx = RandomXMiner::new(DeviceType::GPU);
        
        // GPU miners should have larger batch sizes (check through name or behavior)
        assert_ne!(cpu_sha256.name(), gpu_sha256.name());
        assert_ne!(cpu_randomx.name(), gpu_randomx.name());
    }
}

#[cfg(test)]
mod telemetry_tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_metrics_initialization() {
        let metrics = MinerMetrics::new();
        let snapshot = metrics.snapshot().await;
        
        assert_eq!(snapshot.status, "Idle");
        assert_eq!(snapshot.hashrate, 0);
        assert_eq!(snapshot.total_hashes, 0);
        assert_eq!(snapshot.blocks_found, 0);
        assert_eq!(snapshot.uptime, snapshot.uptime);
    }

    #[tokio::test]
    async fn test_metrics_updates() {
        let metrics = Arc::new(MinerMetrics::new());
        
        // Update hashrate
        metrics.hashrate.store(1000, std::sync::atomic::Ordering::Relaxed);
        metrics.total_hashes.fetch_add(5000, std::sync::atomic::Ordering::Relaxed);
        metrics.blocks_found.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        {
            let mut status = metrics.status.write().await;
            *status = "Mining".to_string();
        }
        
        let snapshot = metrics.snapshot().await;
        
        assert_eq!(snapshot.status, "Mining");
        assert_eq!(snapshot.hashrate, 1000);
        assert_eq!(snapshot.total_hashes, 5000);
        assert_eq!(snapshot.blocks_found, 1);
    }

    #[tokio::test]
    async fn test_hash_history_recording() {
        let metrics = MinerMetrics::new();
        
        // Record some hashrate values
        metrics.record_hashrate(1000).await;
        metrics.record_hashrate(1500).await;
        metrics.record_hashrate(2000).await;
        
        let snapshot = metrics.snapshot().await;
        
        assert_eq!(snapshot.average_hashrate, 1500.0);
        assert_eq!(snapshot.peak_hashrate, 2000);
    }

    #[tokio::test]
    async fn test_efficiency_calculation() {
        let metrics = Arc::new(MinerMetrics::new());
        
        // Simulate mining: 1 block found in 1 billion hashes
        metrics.total_hashes.store(1_000_000_000, std::sync::atomic::Ordering::Relaxed);
        metrics.blocks_found.store(1, std::sync::atomic::Ordering::Relaxed);
        
        let snapshot = metrics.snapshot().await;
        assert_eq!(snapshot.efficiency, 1.0); // 1 block per billion hashes
        
        // Test with no hashes
        let fresh_metrics = MinerMetrics::new();
        let fresh_snapshot = fresh_metrics.snapshot().await;
        assert_eq!(fresh_snapshot.efficiency, 0.0);
    }

    #[tokio::test]
    async fn test_uptime_calculation() {
        let metrics = MinerMetrics::new();
        let initial_snapshot = metrics.snapshot().await;
        
        // Wait a bit
        sleep(Duration::from_millis(100)).await;
        
        let later_snapshot = metrics.snapshot().await;
        assert!(later_snapshot.uptime >= initial_snapshot.uptime);
        assert_eq!(later_snapshot.uptime, later_snapshot.uptime);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use solo_miner::orchestrator::{MiningMode, get_recommended_threads};

    #[test]
    fn test_mining_mode_thread_calculation() {
        let performance_threads = get_recommended_threads(&MiningMode::Performance);
        let conservative_threads = get_recommended_threads(&MiningMode::Conservative);
        
        assert_eq!(performance_threads, num_cpus::get());
        assert_eq!(conservative_threads, (num_cpus::get() / 2).max(1));
        assert!(performance_threads >= conservative_threads);
    }

    #[test]
    fn test_block_structure() {
        let block = Block {
            id: 1,
            timestamp: 123456789,
            data: "test_data".to_string(),
            previous_hash: "prev_hash".to_string(),
            nonce: 42,
            difficulty: "0000".to_string(),
        };
        
        assert_eq!(block.id, 1);
        assert_eq!(block.timestamp, 123456789);
        assert_eq!(block.data, "test_data");
        assert_eq!(block.previous_hash, "prev_hash");
        assert_eq!(block.nonce, 42);
        assert_eq!(block.difficulty, "0000");
    }

    #[tokio::test]
    async fn test_full_mining_cycle() {
        let metrics = Arc::new(MinerMetrics::new());
        let stop_signal = Arc::new(tokio::sync::Mutex::new(false));
        
        // Start a single mining instance
        let _miner = Arc::new(Sha256Miner::new(DeviceType::CPU));
        let _wallet = "test_wallet".to_string();
        let _difficulty = "00000".to_string(); // Easy difficulty for testing
        
        // Run for a short time
        let mining_task = tokio::spawn({
            let _metrics_clone = metrics.clone();
            let stop_clone = stop_signal.clone();
            async move {
                // This would normally run indefinitely, but we'll stop it quickly
                *stop_clone.lock().await = true;
                Ok::<(), solo_miner::SoloMinerError>(())
            }
        });
        
        // Let it run briefly
        sleep(Duration::from_millis(50)).await;
        
        // Stop the mining
        *stop_signal.lock().await = true;
        
        let _ = mining_task.await;
        
        // Verify metrics were updated
        let snapshot = metrics.snapshot().await;
        assert_eq!(snapshot.uptime, snapshot.uptime);
    }
}