use super::{block, ledger, transaction};
pub trait Protocol<T, B, L>
where
    T: transaction::Transaction,
    B: block::Block<T>,
    L: ledger::Legder<T, B>,
{
    type Key;
    fn verify_ledger(&self, ledger: &L) -> bool;
    fn verify_block(&self, block: &B) -> bool;
    fn verify_transaction(&self, transaction: &T) -> bool;
    fn generate_wallet(&self, ledger: &L, pub_key: &T::PublicKey) -> T::CoinType;
    fn is_transaction_confirmed(&self, ledger: &L, transaction_hash: &T::TransactionHash) -> bool;
    fn generate_transaction(
        &self,
        owner: Self::Key,
        reciver: T::PublicKey,
        coins: T::CoinType,
    ) -> T;
}
