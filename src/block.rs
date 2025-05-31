use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    index: u64,
    previous_hash: String,
    timestamp: String,
    tx: Vec<Transaction>,
    nonce: u64,
    hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: String, tx: Vec<Transaction>, nonce: u64, hash: String) -> Self {
        Block {
            index,
            previous_hash,
            timestamp,
            tx,
            nonce,
            hash,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Block {{ index: {}, previous_hash: {}, timestamp: {}, tx: {}, nonce: {}, hash: {} }}",
            self.index, self.previous_hash, self.timestamp, self.tx.iter().map(|tx| tx.to_string()).collect::<String>(), self.nonce, self.hash
        )
    }

    pub fn get_previous_hash(&self) -> &String {
        &self.previous_hash
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}