pub mod miner;
pub use miner::MinerResult;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum DeviceType {
    CPU,
    GPU,
}

pub trait MinerAlgorithm: Send + Sync {
    fn mine(&self, block: &mut Block) -> MinerResult<Option<String>>;
    fn name(&self) -> &'static str;
}
