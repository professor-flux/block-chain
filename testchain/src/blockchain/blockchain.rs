extern crate chrono;
extern crate hex;
extern crate openssl;

extern crate serde;
extern crate serde_json;

use std::fmt::Display;

use super::transaction::Transaction;
use openssl::sha;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    index: u32,
    previous_hash: String,
    nonce: u64,
    data: Vec<Transaction>,
    // time: DateTime<Utc>,
}

impl Block {
    pub fn new(index: u32, data: Vec<Transaction>) -> Self {
        Self {
            index: index,
            previous_hash: hex::encode([0; 32]),
            nonce: 0,
            data: data,
            // time: Utc::now(),
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
        write!(f, "Block {}", serde_json::to_string_pretty(self).unwrap())
    }
}

#[test]
fn test_block() {
    let mut blk = Block::new(0, Vec::new());
    blk.mine_me_onii_chan(2);
    println!("{}", blk);
    println!("{:?}", blk.hash());
}

pub struct BlockChain {
    difficulty: usize,
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new(difficulty: usize) -> Self {
        Self {
            difficulty: difficulty,
            chain: vec![Block::new(0, Vec::new())],
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
    let mut blkchn = BlockChain::new(3);

    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;

    let alice = Rsa::generate(1024).unwrap();
    let alice = PKey::from_rsa(alice).unwrap();

    let bob = Rsa::generate(1024).unwrap();
    let bob = PKey::from_rsa(bob).unwrap();

    let mut tlist = vec![];
    {
        let from = String::from_utf8(alice.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let to = String::from_utf8(bob.public_key_to_pem().unwrap()).unwrap();

        let t = Transaction::make_transaction(from, to);
        tlist.push(t);
    }
    {
        let from = String::from_utf8(bob.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let to = String::from_utf8(bob.public_key_to_pem().unwrap()).unwrap();

        let t = Transaction::make_transaction(from, to);
        tlist.push(t);
    }
    {
        let from = String::from_utf8(bob.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let to = String::from_utf8(alice.public_key_to_pem().unwrap()).unwrap();

        let t = Transaction::make_transaction(from, to);
        tlist.push(t);
    }
    {
        let from = String::from_utf8(alice.private_key_to_pem_pkcs8().unwrap()).unwrap();
        let to = String::from_utf8(bob.public_key_to_pem().unwrap()).unwrap();

        let t = Transaction::make_transaction(from, to);
        tlist.push(t);
    }

    blkchn.add_block(Block::new(1, tlist.clone()));
    blkchn.add_block(Block::new(2, tlist.clone()));
    blkchn.add_block(Block::new(3, tlist.clone()));
    blkchn.add_block(Block::new(4, tlist.clone()));

    for b in blkchn.get_chain() {
        println!("{}", b);
        println!("hash: {:?}", b.hash());
        println!();
    }
}
