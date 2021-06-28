use super::types::*;

use super::{block::Block, transaction::Transaction};
pub struct Ledger {
    _block_size: usize,
    difficulty: usize,
    chain: Vec<Block>,
}

impl Ledger {
    pub fn new(genesis_block_owner: PubKey, difficulty: usize, block_size: usize) -> Self {
        let mut list = vec![];
        for i in 0..block_size {
            list.push(Transaction::genesis_transaction(
                genesis_block_owner.clone(),
                i.to_string(),
            ));
        }

        Self {
            _block_size: block_size,
            difficulty,
            chain: vec![Block::new(list)],
        }
    }

    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn get_chain(&self) -> &[Block] {
        &self.chain
    }

    pub fn push_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}
