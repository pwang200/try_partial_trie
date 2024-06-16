use partial_binary_merkle::Hash;
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    TRIE_ELF, TRIE_ID,
};
use risc0_zkvm::{default_prover, ExecutorEnv};

// AMD Ryzen Threadripper 3960X 24-Core, 14m6.681s, for 128 init_usize and 128 additions
fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    // tracing_subscriber::fmt()
    //     .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
    //     .init();

    let init_size = 12usize;
    let n_additions  = 12usize;
    let mut input = common::Input::new(init_size, n_additions);
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
    let host_result = input.process();
    println!("host : {:?}", host_result);
    assert_eq!(output, host_result);
}
