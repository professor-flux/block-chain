extern crate chrono;
extern crate hex;
extern crate openssl;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

use std::fmt::Display;

use openssl::hash;
use openssl::pkey::PKey;
use openssl::sign;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    from: String,
    to: String,
    signature: String,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction {}",
            serde_json::to_string_pretty(self).unwrap()
        )
    }
}

impl Transaction {
    fn fromat_transaction_for_signing(&self) -> String {
        format!("{}{}", self.from, self.to,)
    }

    pub fn make_transaction(from: String, to: String) -> Self {
        use openssl::hash::MessageDigest;
        use openssl::sign::Signer;
        let from = PKey::private_key_from_pem(from.as_bytes()).unwrap();
        let pubkey_from = String::from_utf8(from.public_key_to_pem().unwrap()).unwrap();

        let mut tranaction = Self {
            from: pubkey_from,
            to,
            signature: String::new(),
        };

        let fmt_transaction = tranaction.fromat_transaction_for_signing();

        let mut signer = Signer::new(MessageDigest::sha256(), &from).unwrap();
        signer.update(fmt_transaction.as_bytes()).unwrap();
        let signature = hex::encode(signer.sign_to_vec().unwrap());
        tranaction.signature = signature;
        tranaction
    }

    pub fn new(from: String, to: String, signature: String) -> Self {
        Self {
            from,
            to,
            signature,
        }
    }

    pub fn verify(&self) -> bool {
        let from = PKey::public_key_from_pem(self.from.as_bytes()).unwrap();
        let mut verifier = sign::Verifier::new(hash::MessageDigest::sha256(), &from).unwrap();

        let transaction = self.fromat_transaction_for_signing();
        verifier.update(transaction.as_bytes()).unwrap();
        verifier
            .verify(hex::decode(self.signature.as_bytes()).unwrap().as_slice())
            .unwrap()
    }
}

#[test]
fn test_tranaction() {
    use openssl::rsa::Rsa;

    let keypair = Rsa::generate(2048).unwrap();
    let keypair = PKey::from_rsa(keypair).unwrap();

    let from = String::from_utf8(keypair.private_key_to_pem_pkcs8().unwrap()).unwrap();
    let to = String::from_utf8(keypair.public_key_to_pem().unwrap()).unwrap();

    let transaction = format!("Transaction {{\nfrom:\n{},\nto:\n{}\n}}", from, to,);
    println!("{}", transaction);

    let t = Transaction::make_transaction(from, to);
    assert!(t.verify());
}
