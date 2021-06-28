use super::types::*;

use super::{ledger::Ledger, transaction::Transaction};

pub fn verify_ledger(ledger: &Ledger) -> bool {
    let mut wallet = Wallet::new();

    let mut previous_hash = BlockHash::new();
    if let Some(genesis_block) = ledger.get_chain().first() {
        for genesis_transaction in genesis_block.transaction_list() {
            wallet
                .entry(genesis_transaction.owner_public_key().clone())
                .or_default()
                .insert(genesis_transaction.hash());
        }
    } else {
        return false;
    }

    for block in ledger.get_chain() {
        if *block.previous_block_hash() != previous_hash {
            return false;
        }
        if !block.verify_nonce(ledger.difficulty()) {
            return false;
        }

        for transaction in block.transaction_list() {
            if !transaction.verify_signature() {
                return false;
            }

            if let Some(coins) = wallet.get_mut(transaction.owner_public_key()) {
                if !coins.contains(transaction.previous_transaction_hash()) {
                    return false;
                }
                coins.remove(transaction.previous_transaction_hash());

                wallet
                    .entry(transaction.reciver_public_key().clone())
                    .or_default()
                    .insert(transaction.hash());
            } else {
                return false;
            }
        }

        previous_hash = block.hash();
    }
    true
}

use std::collections::HashSet;
pub fn verify_transaction(
    transaction: &Transaction,
    free_coins: &HashSet<TransactionHash>,
) -> bool {
    if transaction.verify_signature() {
        free_coins.contains(transaction.previous_transaction_hash())
    } else {
        false
    }
}

pub fn get_balance(ledger: &Ledger, pub_key: &PubKey) -> Coin {
    let mut balance: Coin = 0;
    for block in ledger.get_chain() {
        for transaction in block.transaction_list() {
            if transaction.owner_public_key() == pub_key {
                balance -= 1;
            }
            if transaction.reciver_public_key() == pub_key {
                balance += 1;
            }
        }
    }
    balance
}

pub fn generate_wallet(ledger: &Ledger) -> Result<Wallet, ()> {
    let mut wallet: Wallet = Wallet::new();

    for block in ledger.get_chain() {
        for transaction in block.transaction_list() {
            wallet
                .entry(transaction.owner_public_key().clone())
                .or_default()
                .remove(transaction.previous_transaction_hash());

            wallet
                .entry(transaction.reciver_public_key().clone())
                .or_default()
                .insert(transaction.hash());
        }
    }

    Ok(wallet)
}

pub fn is_transaction_confirmed(_ledger: &Ledger, _transaction_hash: &TransactionHash) -> bool {
    todo!();
}

pub fn generate_key() -> PrvKey {
    use openssl::pkey::PKey;
    use openssl::rsa::Rsa;

    let key = Rsa::generate(1024).unwrap();
    let key = PKey::from_rsa(key).unwrap();

    PrvKey::from_utf8(key.private_key_to_pem_pkcs8().unwrap()).unwrap()
}

pub fn prv_key_to_pub_key(prv_key: &PrvKey) -> PubKey {
    use openssl::pkey::PKey;
    let key = PKey::private_key_from_pem(prv_key.as_bytes()).unwrap();
    String::from_utf8(key.public_key_to_pem().unwrap()).unwrap()
}

pub fn generate_transaction(
    wallet: &Wallet,
    owner: PrvKey,
    reciver: PubKey,
    _coins: Coin,
) -> Option<Transaction> {
    let owner_pubkey = prv_key_to_pub_key(&owner);

    if let Some(free_coins) = wallet.get(&owner_pubkey) {
        if let Some(input) = free_coins.iter().next() {
            Some(Transaction::make_transaction(owner, reciver, input.clone()))
        } else {
            None
        }
    } else {
        None
    }
}
