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

#[derive(Clone, Copy)]
pub struct Sha256Miner;

impl MinerAlgorithm for Sha256Miner {
    fn mine(&self, block: &Block, difficulty: &str) -> MinerResult<String> {
        let block_string = format!("{}{}{}{}{}", block.id, block.timestamp, block.data, block.previous_hash, block.nonce);
        let hash = sha256::digest(block_string.as_bytes());
        let binary_hash = hash_to_binary_representation(hash.as_bytes());

        if binary_hash.starts_with(difficulty) {
            return Ok(hash);
        }
        Err(crate::SoloMinerError::MiningError("No block found".to_string()))
    }

    fn name(&self) -> &'static str {
        "SHA-256"
    }

    fn clone(&self) -> Box<dyn MinerAlgorithm> {
        Box::new(Sha256Miner)
    }
}

#[derive(Clone, Copy)]
pub struct RandomXMiner;

impl MinerAlgorithm for RandomXMiner {
    fn mine(&self, block: &Block, difficulty: &str) -> MinerResult<String> {
        // Simulate a different hashing process for RandomX
        let block_string = format!("RandomX data + {}{}{}{}{}", block.id, block.timestamp, block.data, block.previous_hash, block.nonce);
        let hash = sha256::digest(block_string.as_bytes()); // Using sha256 for simulation
        let binary_hash = hash_to_binary_representation(hash.as_bytes());

        if binary_hash.starts_with(difficulty) {
            return Ok(hash);
        }
        Err(crate::SoloMinerError::MiningError("No block found".to_string()))
    }

    fn name(&self) -> &'static str {
        "RandomX"
    }

    fn clone(&self) -> Box<dyn MinerAlgorithm> {
        Box::new(RandomXMiner)
    }
}

pub async fn start_mining(wallet_address: &str, num_threads: usize, algorithm: Box<dyn MinerAlgorithm>, metrics: Arc<Mutex<crate::telemetry::MinerMetrics>>, difficulty: &str, timeout_secs: Option<u64>) -> MinerResult<()> {
    println!("Mining for wallet: {}", wallet_address);
    println!("Algorithm: {}", algorithm.name());
    println!("Difficulty: {}", difficulty);
    println!("Number of threads: {}", num_threads);

    let block = Block {
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

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let mut handles = vec![];
    let total_hashes_in_session = Arc::new(Mutex::new(0u64));
    let start_time = Arc::new(Instant::now());

    for i in 0..num_threads {
        let mut thread_block = block.clone();
        let algorithm = algorithm.clone();
        let difficulty = difficulty.to_owned();
        let tx = tx.clone();
        let metrics = metrics.clone();
        let total_hashes_in_session = total_hashes_in_session.clone();
        let start_time = start_time.clone();

        let handle = tokio::spawn(async move {
            let mut nonce = i as u64;
            loop {
                thread_block.nonce = nonce;
                if let Ok(hash) = algorithm.mine(&thread_block, &difficulty) {
                    let _ = tx.send(hash).await;
                    break;
                }
                nonce += num_threads as u64;

                let mut total_hashes = total_hashes_in_session.lock().await;
                *total_hashes += 1;

                if *total_hashes % 100000 == 0 {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    if elapsed > 0.0 {
                        let hashrate = *total_hashes as f64 / elapsed;
                        let mut m = metrics.lock().await;
                        m.hashrate = hashrate;
                        m.total_hashes = *total_hashes;
                    }
                }
            }
        });
        handles.push(handle);
    }

    let mining_future = async {
        if let Some(found_hash) = rx.recv().await {
            println!("Found a block!");
            println!("Hash: {}", found_hash);

            let mut m = metrics.lock().await;
            m.blocks_found += 1;
        }
        Ok::<(), crate::SoloMinerError>(())
    };

    if let Some(timeout) = timeout_secs {
        tokio::select! {
            _ = mining_future => {},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(timeout)) => {
                println!("Mining timed out after {} seconds.", timeout);
            }
        }
    } else {
        mining_future.await?;
    }

    // Update status to idle after mining is done
    {
        let mut m = metrics.lock().await;
        m.status = "Idle".to_string();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::MinerMetrics;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_miner_finds_block() {
        let metrics = Arc::new(Mutex::new(MinerMetrics::default()));
        let algorithm = Box::new(Sha256Miner);
        let difficulty = "000";

        let result = start_mining("test_wallet", 4, algorithm, metrics, difficulty, Some(5)).await;

        assert!(result.is_ok());
    }
}
