use risc0_zkvm::{default_prover, ExecutorEnv, InnerReceipt};

use methods::{
    MERKLE_ELF, MERKLE_ID,
};
use crate::data::get_long_merkle_proof;

mod data;

fn stark_pipeline() {
    let now = || std::time::Instant::now();

    let env = ExecutorEnv::builder()
        .write(&get_long_merkle_proof())
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let proving_start = now();
    let receipt = prover
        .prove(env, MERKLE_ELF)
        .unwrap();
    let proving_time = now().duration_since(proving_start);

    let merkle_proof_validity: bool = receipt.journal.decode().unwrap();
    assert!(merkle_proof_validity, "Merkle proof is invalid");

    let verifying_start = now();
    receipt
        .verify(MERKLE_ID)
        .unwrap();
    let verifying_time = now().duration_since(verifying_start);

    let InnerReceipt::Composite(composite) = receipt.inner else {
        panic!("Expected composite receipt");
    };
    println!("We got a composite receipt with {} segments. Total size of STARKs is {} kB",
             composite.segments.len(),
             composite.segments.iter().map(|s| s.seal.len()).sum::<usize>() / 256 // seal is over u32
    );
    println!("Proving time: {proving_time:?}, Verifying time: {verifying_time:?}");
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    stark_pipeline();
}
