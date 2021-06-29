pub type Coin = u64;
pub type PubKey = String;
pub type PrvKey = String;
pub type TransactionHash = String;
pub type Signature = String;
pub type BlockHash = String;

use std::collections::{HashMap, HashSet};
pub type Wallet = HashMap<PubKey, HashSet<TransactionHash>>;
