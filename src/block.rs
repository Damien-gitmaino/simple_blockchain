use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub(crate) index: u64,
    previous_hash: String,
    timestamp: String,
    pub(crate) tx: Vec<Transaction>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    fn mock_transaction(id: f64) -> Transaction {
        Transaction::new(format!("from{id}"), format!("to{id}"), id * 10.0)
    }

    #[test]
    fn test_block_creation() {
        let txs = vec![
            mock_transaction(1.0),
            mock_transaction(2.0),
        ];

        let block = Block::new(
            1,
            "prev_hash_123".to_string(),
            "2025-05-31T12:00:00Z".to_string(),
            txs.clone(),
            42,
            "current_hash_abc".to_string(),
        );

        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, "prev_hash_123");
        assert_eq!(block.timestamp, "2025-05-31T12:00:00Z");
        assert_eq!(block.tx.len(), 2);
        assert_eq!(block.nonce, 42);
        assert_eq!(block.hash, "current_hash_abc");
    }

    #[test]
    fn test_block_to_string() {
        let txs = vec![
            mock_transaction(1.0),
            mock_transaction(2.0),
        ];

        let block = Block::new(
            2,
            "hash_xyz".to_string(),
            "2025-05-31T15:30:00Z".to_string(),
            txs.clone(),
            99,
            "hash_final".to_string(),
        );

        let block_str = block.to_string();

        assert!(block_str.contains("index: 2"));
        assert!(block_str.contains("previous_hash: hash_xyz"));
        assert!(block_str.contains("timestamp: 2025-05-31T15:30:00Z"));
        assert!(block_str.contains("nonce: 99"));
        assert!(block_str.contains("hash: hash_final"));

        // Optional: Ensure the transaction to_string is included
        for tx in txs {
            assert!(block_str.contains(&tx.to_string()));
        }
    }

    #[test]
    fn test_getters() {
        let block = Block::new(
            3,
            "prev_hash".to_string(),
            "2025-05-31T00:00:00Z".to_string(),
            vec![],
            0,
            "final_hash".to_string(),
        );

        assert_eq!(block.get_previous_hash(), "prev_hash");
        assert_eq!(block.get_hash(), "final_hash");
    }
}