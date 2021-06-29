use super::types::*;

use super::{Ledger, Transaction};

pub fn verify_ledger(ledger: &Ledger) -> Option<Wallet> {
    let mut wallet = Wallet::new();

    let mut previous_hash = BlockHash::new();

    for block in ledger.get_chain() {
        if *block.previous_block_hash() != previous_hash {
            return None;
        }
        if !block.verify_nonce(ledger.difficulty()) {
            return None;
        }

        for transaction in block.transaction_list() {
            if !transaction.verify_signature() {
                return None;
            }

            if let Some(coins) = wallet.get_mut(transaction.owner_public_key()) {
                if !coins.contains(transaction.previous_transaction_hash()) {
                    return None;
                }
                coins.remove(transaction.previous_transaction_hash());

                wallet
                    .entry(transaction.reciver_public_key().clone())
                    .or_default()
                    .insert(transaction.hash());
            } else {
                return None;
            }
        }

        wallet
            .entry(block.miner().clone())
            .or_default()
            .insert(block.hash());

        previous_hash = block.hash();
    }
    Some(wallet)
}

pub fn verify_transaction(transaction: &Transaction, wallet: &Wallet) -> bool {
    if transaction.verify_signature() {
        match wallet.get(transaction.owner_public_key()) {
            Some(owner_wallet) => owner_wallet.contains(transaction.previous_transaction_hash()),
            None => false,
        }
    } else {
        false
    }
}

pub fn get_balance(wallet: &Wallet, pub_key: &PubKey) -> Coin {
    match wallet.get(pub_key) {
        Some(user_wallet) => user_wallet.len() as Coin,
        None => 0,
    }
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

        wallet
            .entry(block.miner().clone())
            .or_default()
            .insert(block.hash());
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

pub fn prvkey_to_pubkey(prv_key: &PrvKey) -> PubKey {
    use openssl::pkey::PKey;
    let key = PKey::private_key_from_pem(prv_key.as_bytes()).unwrap();
    String::from_utf8(key.public_key_to_pem().unwrap()).unwrap()
}

pub fn generate_transaction(
    wallet: &Wallet,
    owner: PrvKey,
    reciver: PubKey,
) -> Option<Transaction> {
    let owner_pubkey = prvkey_to_pubkey(&owner);

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
