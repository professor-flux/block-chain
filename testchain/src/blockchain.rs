extern crate chrono;
extern crate hex;
extern crate openssl;

use std::fmt::Display;

use chrono::{DateTime, Utc};
use openssl::sha;

#[derive(Debug)]
pub struct Block {
    index: u32,
    previous_hash: String,
    nonce: i64,
    data: String,
    time: DateTime<Utc>,
}

impl Block {
    pub fn new(index: u32, data: String) -> Self {
        Self {
            index: index,
            previous_hash: hex::encode([0; 32]),
            nonce: 0,
            data: data,
            time: Utc::now(),
        }
    }

    pub fn hash(&self) -> String {
        self.calculate_hash()
    }

    pub fn set_previous_block_hash(&mut self, previous_hash: String) {
        self.previous_hash = previous_hash;
    }

    pub fn mine_me_onii_chan(&mut self, difficulty: usize) {
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

    fn calculate_hash(&self) -> String {
        let dig = format!("{}", self);
        hex::encode(sha::sha256(dig.as_bytes()))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
                "Block {{\n\tindex: {},\n\tprevious hash: \"{}\",\n\tnonce: {},\n\tdata: \"{}\",\n\ttime: {}\n}}",
                self.index, self.previous_hash, self.nonce, self.data, self.time
            )
    }
}

#[test]
fn test_block() {
    let mut blk = Block::new(0, String::from("Genesis Block"));
    blk.mine_me_onii_chan(2);
    println!("{:?}", blk.hash());
    println!("{:?}", blk);
}

pub struct BlockChain {
    difficulty: usize,
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new(difficulty: usize) -> Self {
        Self {
            difficulty: difficulty,
            chain: vec![Block::new(0, String::from("Genesis Block"))],
        }
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.set_previous_block_hash(self.chain.last().unwrap().hash());
        block.mine_me_onii_chan(self.difficulty);
        self.chain.push(block);
    }

    pub fn get_chain(&self) -> &[Block] {
        &self.chain
    }
}

#[test]
fn test_blockchain() {
    let mut myblockchain = BlockChain::new(4);
    myblockchain.add_block(Block::new(1, String::from("Block Data 1")));
    myblockchain.add_block(Block::new(2, String::from("Block Data 2")));
    myblockchain.add_block(Block::new(3, String::from("Block Data 3")));
    myblockchain.add_block(Block::new(4, String::from("Block Data 4")));

    for b in myblockchain.get_chain() {
        println!("{}", b);
        println!("hash: {:?}", b.hash());
        println!();
    }
}
