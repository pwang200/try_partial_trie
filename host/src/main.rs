use partial_binary_merkle::Hash;
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    TRIE_ELF, TRIE_ID,
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use clap::Parser;

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    init_trie_size: usize,

    #[arg(short, long, default_value_t = 0)]
    new_additions: usize,

    #[arg(long, default_value_t = false)]
    partial_trie: bool,
}

// AMD Ryzen Threadripper 3960X 24-Core, 11m8.399s, for 100 init_usize and 100 additions
fn main() {
    // let init_size = 10000usize;
    // let n_additions  = 100usize;
    let args = Args::parse();

    println!("init trie size: {}, new additions: {}, use partial trie: {}", args.init_trie_size, args.new_additions, args.partial_trie);
    let mut input = common::Input::new(args.init_trie_size, args.new_additions, args.partial_trie);
    assert!(input.trie.verify_partial());
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
    let prover = default_prover();
    let receipt = prover
        .prove(env, TRIE_ELF)
        .unwrap();
    let output: Option<Hash> = receipt.journal.decode().unwrap();
    receipt
        .verify(TRIE_ID)
        .unwrap();
    println!("guest: {:?}", output);
    let host_result = input.verify_and_process();
    println!("host : {:?}", host_result);
    assert_eq!(output, host_result);
}
