use super::{Block, BlockHash, PubKey, Transaction};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    difficulty: usize,
    chain: Vec<Block>,
}

impl Ledger {
    pub fn difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn get_chain(&self) -> &[Block] {
        &self.chain
    }

    pub fn add_block(&mut self, list: Vec<Transaction>, miner: PubKey) -> Result<(), ()> {
        let hash = if let Some(last) = self.chain.last() {
            last.hash()
        } else {
            BlockHash::new()
        };
        let mut block = Block::new(list, hash, miner);
        block.mine(self.difficulty);
        self.chain.push(block);
        Ok(())
    }

    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_string(s: &str) -> Result<Ledger, serde_json::Error> {
        serde_json::from_str(s)
    }
}
