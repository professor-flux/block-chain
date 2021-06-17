use super::transaction;

pub trait Block<T>
where
    T: transaction::Transaction,
{
    const BLOCKSIZE: usize;
    type BlockHash;

    fn previous_block_hash(&self) -> &Self::BlockHash;
    fn hash(&self) -> Self::BlockHash;
    fn transaction_list(&self) -> &[T];
}
