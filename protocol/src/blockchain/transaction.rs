pub trait Transaction {
    type CoinType;
    type PublicKey;
    type Signature;
    type TransactionHash;

    fn owner_public_key(&self) -> &Self::PublicKey;
    fn signature(&self) -> &Self::Signature;
    fn reciver_public_key(&self) -> &Self::PublicKey;
    fn previous_transaction_hash(&self) -> &Self::TransactionHash;
    fn hash(&self) -> Self::TransactionHash;
}
