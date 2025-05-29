use sha2::{Digest, Sha256};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn mine_block(index: u32, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = 0;

        loop {
            let content = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, nonce);
            let hash = calculate_hash(&content);
            if hash.starts_with(&"0".repeat(difficulty)) {
                return Block {
                    index,
                    timestamp,
                    data,
                    previous_hash,
                    hash,
                    nonce,
                };
            }
            nonce += 1;
        }
    }
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}