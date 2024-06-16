#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

// use partial_binary_merkle::{PartialMerkleTrie, Hash, ID};
use common::Input;
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
    let mut input: Input = env::read();
    assert!(input.trie.verify_partial());
    let r = input.process();
    env::commit(&r);
}
