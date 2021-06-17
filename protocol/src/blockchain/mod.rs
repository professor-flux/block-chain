pub mod block;
pub mod ledger;
pub mod protocol;
pub mod transaction;

pub enum NodeType {
    Wallet,
    Miner,
}
