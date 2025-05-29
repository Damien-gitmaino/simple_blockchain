use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis = Block::mine_block(0, "Genesis Block".into(), "0".into(), difficulty);
        Self {
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last = self.chain.last().unwrap();
        let new_block = Block::mine_block(last.index + 1, data, last.hash.clone(), self.difficulty);
        self.chain.push(new_block);
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }
}