#![no_main]

use common::Input;
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
    let start = env::cycle_count();
    let mut input: Input = env::read();
    let r = input.verify_and_process();
    env::commit(&r);
    let end = env::cycle_count();
    eprintln!("cycle count: {}", end - start);
}
