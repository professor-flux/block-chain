///FluxCoin

/// type aliasing for components of Transaction
pub mod types;

pub mod block;
pub mod ledger;
pub mod transaction;

pub use block::Block;
pub use ledger::Ledger;
pub use transaction::Transaction;
pub use types::*;

///Protocol functions - for making transaction, verifying ledger and transaction etc.
pub mod protocol;

mod test {
    #[test]
    fn check_ledger_txt() {
        use crate::protocol::verify_ledger;
        use crate::Ledger;

        fn read_ledger() -> Ledger {
            use std::fs::File;
            let mut ldg_file = File::open("ledger.txt").unwrap();

            use std::io::Read;
            let mut ldg = String::new();
            ldg_file.read_to_string(&mut ldg).unwrap();

            Ledger::from_string(&ldg).unwrap()
        }

        fn write_ledger(ledger: &Ledger) {
            use std::fs::File;
            let mut ldg_file = File::create("ledger.txt").unwrap();

            use std::io::Write;
            writeln!(&mut ldg_file, "{}", ledger.to_string().unwrap()).unwrap();
        }

        let ldg = read_ledger();
        assert!(verify_ledger(&ldg).is_some());
        write_ledger(&ldg);
    }
}
