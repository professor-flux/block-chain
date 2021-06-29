use super::types::*;

use openssl::pkey::PKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    from: PubKey,
    to: PubKey,
    input: TransactionHash,
    signature: Signature,
}

impl Transaction {
    fn fromat_transaction_for_signing(&self) -> String {
        format!("{}\n{}\n{}", self.from, self.to, self.from)
    }

    fn calculate_hash(&self) -> BlockHash {
        use openssl::sha;
        let fmt_transaction = self.fromat_transaction_for_signing();
        hex::encode(sha::sha256(fmt_transaction.as_bytes()))
    }
}

impl Transaction {
    pub fn make_transaction(from: PrvKey, to: PubKey, input: TransactionHash) -> Self {
        use openssl::hash::MessageDigest;
        use openssl::sign::Signer;

        let from = PKey::private_key_from_pem(from.as_bytes()).unwrap();
        let pubkey_from = String::from_utf8(from.public_key_to_pem().unwrap()).unwrap();

        let mut tranaction = Self {
            from: pubkey_from,
            to,
            input,
            signature: String::new(),
        };

        let fmt_transaction = tranaction.fromat_transaction_for_signing();

        let mut signer = Signer::new(MessageDigest::sha256(), &from).unwrap();
        signer.update(fmt_transaction.as_bytes()).unwrap();
        let signature = hex::encode(signer.sign_to_vec().unwrap());
        tranaction.signature = signature;

        tranaction
    }

    pub fn owner_public_key(&self) -> &PubKey {
        &self.from
    }

    pub fn reciver_public_key(&self) -> &PubKey {
        &self.to
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn previous_transaction_hash(&self) -> &TransactionHash {
        &self.input
    }

    pub fn hash(&self) -> TransactionHash {
        self.calculate_hash()
    }

    pub fn verify_signature(&self) -> bool {
        use openssl::hash::MessageDigest;
        use openssl::sign::Verifier;

        let from = PKey::public_key_from_pem(self.from.as_bytes()).unwrap();
        let mut verifier = Verifier::new(MessageDigest::sha256(), &from).unwrap();

        let transaction = self.fromat_transaction_for_signing();
        verifier.update(transaction.as_bytes()).unwrap();
        verifier
            .verify(hex::decode(self.signature.as_bytes()).unwrap().as_slice())
            .unwrap()
    }
}
