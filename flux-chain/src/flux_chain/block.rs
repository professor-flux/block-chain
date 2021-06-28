use super::transaction::Transaction;
use super::types::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pervious_block: BlockHash,
    nonce: u64,
    transaction_list: Vec<Transaction>,
}

impl Block {
    fn format_block_for_hash(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn calculate_hash(&self) -> BlockHash {
        use openssl::sha;
        let fmt_block = self.format_block_for_hash();
        hex::encode(sha::sha256(fmt_block.as_bytes()))
    }
}

impl Block {
    pub fn new(list: Vec<Transaction>) -> Self {
        Self {
            pervious_block: BlockHash::new(),
            nonce: 0,
            transaction_list: list,
        }
    }

    pub fn previous_block_hash(&self) -> &BlockHash {
        &self.pervious_block
    }
    pub fn set_previous_block_hash(&mut self, link: BlockHash) {
        self.pervious_block = link;
    }

    pub fn mine(&mut self, difficulty: usize) {
        while self
            .calculate_hash()
            .chars()
            .take_while(|c| *c == '0')
            .count()
            < difficulty
        {
            self.nonce += 1;
        }
    }

    pub fn verify_nonce(&self, difficulty: usize) -> bool {
        self.calculate_hash()
            .chars()
            .take_while(|c| *c == '0')
            .count()
            >= difficulty
    }

    pub fn hash(&self) -> BlockHash {
        self.calculate_hash()
    }

    pub fn transaction_list(&self) -> &[Transaction] {
        &self.transaction_list
    }
}
