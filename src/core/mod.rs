pub mod miner;
pub use miner::MinerResult;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
}

#[async_trait::async_trait]
pub trait MinerAlgorithm: Send + Sync {
    async fn mine(&self, block: &mut Block, difficulty: &str, metrics: std::sync::Arc<tokio::sync::Mutex<crate::telemetry::MinerMetrics>>) -> MinerResult<String>;
    fn name(&self) -> &'static str;
}
