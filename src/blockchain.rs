use crate::block::Block;
use crate::transaction::Transaction;
use sha2::Digest;
use chrono::prelude::*;

#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    mempool: Vec<Transaction>,
    difficulty: u64,
}

impl Blockchain {
    pub fn new(difficulty: u64) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let genesis_block = Block::new(0, "0".to_string(), timestamp, vec![], 0, "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
            mempool: vec![],
            difficulty,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.chain
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.mempool.push(tx);
    }

    pub fn mine_block(&mut self) -> String {
        let index = self.chain.len() as u64;
        let previous_hash = self.chain.last().unwrap().get_previous_hash().clone();

        let timestamp = Utc::now().to_rfc3339();
        let tx = self.mempool.clone();

        if tx.is_empty() {
            println!("No transactions to mine.");
            return "No transactions to mine.".to_string();
        }

        let new_block = self.hash_block(index, previous_hash, timestamp.clone(), tx);
        self.add_block(new_block.clone());
        self.mempool.clear(); // Clear mempool after mining
        new_block.get_hash().clone()
    }

    pub fn calculate_hash(&self, index: u64, previous_hash: &str, timestamp: String, tx: &[Transaction], nonce: u64) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            index,
            previous_hash,
            timestamp,
            tx.iter().map(|tx| tx.to_string()).collect::<String>(),
            nonce
        );

        let mut hasher = sha2::Sha256::new();
        hasher.update(block_string);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn hash_block(&self, index: u64, previous_hash: String, timestamp: String, tx: Vec<Transaction>) -> Block {
        let mut nonce = 0;
        loop {
            let hash = self.calculate_hash(index, &previous_hash, timestamp.clone(), &tx, nonce);
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                return Block::new(index, previous_hash, timestamp, tx, nonce, hash);
            }
            nonce += 1;
        }
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
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
    fn test_genesis_block_creation() {
        let blockchain = Blockchain::new(2);
        let genesis_block = &blockchain.chain[0];

        assert_eq!(genesis_block.get_previous_hash(), "0");
        assert_eq!(genesis_block.get_hash(), "0");
        assert_eq!(genesis_block.tx.len(), 0);
        assert_eq!(genesis_block.index, 0);
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new(2);
        let tx = mock_transaction(1.0);

        blockchain.add_transaction(tx.clone());

        assert_eq!(blockchain.mempool.len(), 1);
        assert_eq!(blockchain.mempool[0].to_string(), tx.to_string());
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new(1);
        let block = Block::new(1, "abc".to_string(), "2025-05-31T00:00:00Z".to_string(), vec![], 0, "xyz".to_string());
        blockchain.add_block(block.clone());

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.chain[1].get_hash(), &"xyz".to_string());
    }

    #[test]
    fn test_mine_block_with_transactions() {
        let mut blockchain = Blockchain::new(2); // Low difficulty for test
        blockchain.add_transaction(mock_transaction(1.0));
        blockchain.add_transaction(mock_transaction(2.0));

        let mined_hash = blockchain.mine_block();

        assert!(mined_hash.starts_with("00"));
        assert_eq!(blockchain.chain.len(), 2); // One block mined after genesis
        assert_eq!(blockchain.mempool.len(), 0); // Mempool should be cleared
    }

    #[test]
    fn test_mine_block_without_transactions() {
        let mut blockchain = Blockchain::new(2);
        let result = blockchain.mine_block();

        assert_eq!(result, "No transactions to mine.");
        assert_eq!(blockchain.chain.len(), 1); // No block mined
    }

    #[test]
    fn test_hash_block_respects_difficulty() {
        let blockchain = Blockchain::new(2);
        let block = blockchain.hash_block(
            1,
            "000".to_string(),
            "2025-05-31T12:00:00Z".to_string(),
            vec![mock_transaction(1.0)],
        );

        assert!(block.get_hash().starts_with("00"));
    }

    #[test]
    fn test_calculate_hash_is_deterministic() {
        let blockchain = Blockchain::new(1);
        let txs = vec![mock_transaction(1.0)];
        let timestamp = "2025-05-31T12:00:00Z".to_string();
        let hash1 = blockchain.calculate_hash(1, "prevhash", timestamp.clone(), &txs, 42);
        let hash2 = blockchain.calculate_hash(1, "prevhash", timestamp, &txs, 42);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_chain_integrity() {
        let mut blockchain = Blockchain::new(1);
        blockchain.add_transaction(mock_transaction(1.0));
        blockchain.mine_block();
        blockchain.add_transaction(mock_transaction(2.0));
        blockchain.mine_block();

        assert_eq!(blockchain.get_chain().len(), 3);
        for i in 0..blockchain.get_chain().len() {
            assert_eq!(blockchain.get_blocks()[i].index, i as u64);
        }
    }
}