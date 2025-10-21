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

pub trait MinerAlgorithm: Send + Sync {
    fn mine(&self, block: &Block, difficulty: &str) -> MinerResult<String>;
    fn name(&self) -> &'static str;
    fn clone(&self) -> Box<dyn MinerAlgorithm>;
}
