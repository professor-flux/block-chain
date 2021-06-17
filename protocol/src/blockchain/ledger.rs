use super::{block, transaction};

pub trait Legder<T, B>
where
    T: transaction::Transaction,
    B: block::Block<T>,
{
    fn new() -> Self;
    fn get_top(&self) -> Option<&B>;
    fn get_chain(&self) -> &[B];
}
