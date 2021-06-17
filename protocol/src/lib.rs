mod blockchain;
pub use blockchain::*;

mod test {

    #[test]

    fn test() {
        use crate::block::Block;
        use crate::ledger::Legder;
        // use crate::protocol::Protocol;
        use crate::transaction::Transaction;
        struct MyTransaction {
            from: u32,
            to: u32,
            prv_hash: u32,
            sign: u32,
        }

        impl Transaction for MyTransaction {
            type CoinType = u32;
            type PublicKey = u32;
            type Signature = u32;
            type TransactionHash = u32;

            fn owner_public_key(&self) -> &Self::PublicKey {
                &(self.from)
            }

            fn signature(&self) -> &Self::Signature {
                &(self.sign)
            }

            fn previous_transaction_hash(&self) -> &Self::TransactionHash {
                &(self.prv_hash)
            }

            fn reciver_public_key(&self) -> &Self::PublicKey {
                &(self.to)
            }

            fn hash(&self) -> Self::TransactionHash {
                (self.from) ^ (self.to) ^ (self.sign) ^ (self.prv_hash)
            }
        }

        struct MyBlock<const N: usize> {
            list: [MyTransaction; N],
            prv_hash: u32,
        }

        impl<const N: usize> Block<MyTransaction> for MyBlock<N> {
            const BLOCKSIZE: usize = N;
            type BlockHash = u32;
            fn hash(&self) -> Self::BlockHash {
                let mut hash = 0;
                for transaction in &(self.list) {
                    hash ^= transaction.hash();
                }
                hash
            }

            fn transaction_list(&self) -> &[MyTransaction] {
                &(self.list)
            }

            fn previous_block_hash(&self) -> &Self::BlockHash {
                &(self.prv_hash)
            }
        }

        struct MyLegder {
            chain: Vec<MyBlock<10>>,
        }
        impl Legder<MyTransaction, MyBlock<10>> for MyLegder {
            fn new() -> Self {
                MyLegder {
                    chain: Vec::<MyBlock<10>>::new(),
                }
            }

            fn get_top(&self) -> Option<&MyBlock<10>> {
                self.chain.last()
            }

            fn get_chain(&self) -> &[MyBlock<10>] {
                &(self.chain)
            }
        }

        // struct MyProtocol {
        //     number_of_zero: u32,
        // }

        // #[derive(Debug, Clone, Copy)]
        // struct RSAKey {
        //     pub_key: u32,
        //     prv_key: u32,
        // }
    }
}
