use std::time::{Instant, SystemTime, UNIX_EPOCH};
use super::{MinerAlgorithm, Block};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type MinerResult<T> = Result<T, crate::SoloMinerError>;

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res = String::new();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub struct Sha256Miner;

#[async_trait::async_trait]
impl MinerAlgorithm for Sha256Miner {
    async fn mine(&self, block: &mut Block, difficulty: &str, metrics: Arc<Mutex<crate::telemetry::MinerMetrics>>) -> MinerResult<String> {
        let mut total_hashes_in_session = 0u64;
        let start_time = Instant::now();

        loop {
            let block_string = format!("{}{}{}{}{}", block.id, block.timestamp, block.data, block.previous_hash, block.nonce);
            let hash = sha256::digest(block_string.as_bytes());
            let binary_hash = hash_to_binary_representation(hash.as_bytes());

            total_hashes_in_session += 1;

            if total_hashes_in_session % 100000 == 0 {
                let elapsed = start_time.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let hashrate = total_hashes_in_session as f64 / elapsed;
                    let mut m = metrics.lock().await;
                    m.hashrate = hashrate;
                    m.total_hashes = total_hashes_in_session;
                }
            }

            if binary_hash.starts_with(difficulty) {
                let mut m = metrics.lock().await;
                m.blocks_found += 1;
                return Ok(hash);
            }
            block.nonce += 1;
        }
    }

    fn name(&self) -> &'static str {
        "SHA-256"
    }
}

pub struct RandomXMiner;

#[async_trait::async_trait]
impl MinerAlgorithm for RandomXMiner {
    async fn mine(&self, block: &mut Block, difficulty: &str, metrics: Arc<Mutex<crate::telemetry::MinerMetrics>>) -> MinerResult<String> {
        let mut total_hashes_in_session = 0u64;
        let start_time = Instant::now();

        loop {
            // Simulate a different hashing process for RandomX
            let block_string = format!("RandomX data + {}{}{}{}{}", block.id, block.timestamp, block.data, block.previous_hash, block.nonce);
            let hash = sha256::digest(block_string.as_bytes()); // Using sha256 for simulation
            let binary_hash = hash_to_binary_representation(hash.as_bytes());

            total_hashes_in_session += 1;

            if total_hashes_in_session % 100000 == 0 {
                let elapsed = start_time.elapsed().as_secs_f64();
                if elapsed > 0.0 {
                    let hashrate = total_hashes_in_session as f64 / elapsed;
                    let mut m = metrics.lock().await;
                    m.hashrate = hashrate;
                    m.total_hashes = total_hashes_in_session;
                }
            }

            if binary_hash.starts_with(difficulty) {
                let mut m = metrics.lock().await;
                m.blocks_found += 1;
                return Ok(hash);
            }
            block.nonce += 1;
        }
    }

    fn name(&self) -> &'static str {
        "RandomX"
    }
}

pub async fn start_mining(wallet_address: &str, num_threads: usize, algorithm: Box<dyn MinerAlgorithm>, metrics: Arc<Mutex<crate::telemetry::MinerMetrics>>, difficulty: &str) -> MinerResult<()> {
    println!("Mining for wallet: {}", wallet_address);
    println!("Algorithm: {}", algorithm.name());
    println!("Difficulty: {}", difficulty);
    println!("Number of threads: {}", num_threads);

    let mut block = Block {
        id: 0,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| crate::SoloMinerError::MiningError(format!("Time error: {}", e)))?.as_secs(),
        data: "First block data".to_string(),
        previous_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        nonce: 0,
    };

    // Update status to mining
    {
        let mut m = metrics.lock().await;
        m.status = format!("Mining {} with {} threads", algorithm.name(), num_threads);
    }

    let mining_task_metrics = metrics.clone(); // Clone metrics for the mining task
    let difficulty_owned = difficulty.to_owned(); // Clone difficulty for the mining task
    let mining_task: tokio::task::JoinHandle<MinerResult<()>> = tokio::spawn(async move {
        let found_hash = algorithm.mine(&mut block, &difficulty_owned, mining_task_metrics).await?;

        println!("Found a block!");
        println!("Block ID: {}", block.id);
        println!("Timestamp: {}", block.timestamp);
        println!("Data: {}", block.data);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Nonce: {}", block.nonce);
        println!("Hash: {}", found_hash);
        println!("Binary Hash: {}", hash_to_binary_representation(found_hash.as_bytes()));
        Ok(())
    });

    // Keep the main task alive while mining is happening
    mining_task.await.map_err(|e| crate::SoloMinerError::MiningError(format!("Mining task failed: {}", e)))??;

    // Update status to idle after mining is done
    {
        let mut m = metrics.lock().await;
        m.status = "Idle".to_string();
    }
    Ok(())
}
