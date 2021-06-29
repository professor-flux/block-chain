# block-chain

### A simple and naive implementation of bitcoin (BlockChain and protocol) in [Rust][https://www.rust-lang.org/]

#### Transaction 
A Transaction consists of owner's public-key, reciver's public-key, and input source of a coin, and digital signature.
* Public-Key (RSA key)
* Input: It is the hash(openssl::sha256) of previous transaction where the owner gets this coin.  
  It can be also be the hash of the block which was mined by owner.
* Digital Signature

#### Block
A Block consists of previous block hash, number of transaction, and list of transaction, nouce and public-key of miner.

#### Ledger
A Ledger is a list of block linked with perivous block by having it hash.
