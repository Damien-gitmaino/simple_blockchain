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

    pub fn get_mempool(&self) -> &Vec<Transaction> {
        &self.mempool
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