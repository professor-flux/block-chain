pub mod flux_chain;
use std::io::Read;

use flux_chain::protocol::verify_ledger;
use flux_chain::Ledger;

fn read_ledger() -> Ledger {
    use std::fs::File;
    let mut ldg_file = File::open("ledger.txt").unwrap();

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

fn main() {
    let ldg = read_ledger();
    assert!(verify_ledger(&ldg).is_some());
    write_ledger(&ldg);
}
