use risc0_zkvm::{get_prover_server, ExecutorEnv, ProverOpts};

use methods::{MERKLE_ELF, MERKLE_ID};

use crate::measure;

pub fn stark_pipeline(env: ExecutorEnv) {
    // ======== Prepare the prover =================================================================
    let prover = get_prover_server(&ProverOpts::default()).unwrap();

    // ======== Generate composite proof ===========================================================
    let (proving_time, receipt) = measure(|| prover.prove(env, MERKLE_ELF).unwrap());
    let composite_receipt = receipt.inner.composite().unwrap();

    // ======== Read journal and ensure that the merkle proof was valid ============================
    assert!(
        receipt.journal.decode::<bool>().unwrap(),
        "Merkle proof is invalid"
    );

    // ======== Verify the composite STARK proof ===================================================
    let (verifying_time, _) = measure(|| receipt.verify(MERKLE_ID).unwrap());

    // ======== Compress the receipt ===============================================================
    let (compression_time, succinct_receipt) =
        measure(|| prover.compress(composite_receipt).unwrap());

    // ======== Verify the succinct STARK proof ====================================================
    let (succinct_verifying_time, _) = measure(|| succinct_receipt.verify_integrity().unwrap());

    // ======== Report the times and sizes =========================================================
    let composite_size = composite_receipt
        .segments
        .iter()
        .map(|s| s.seal.len())
        .sum::<usize>()
        / 256;
    let succinct_size = succinct_receipt.get_seal_bytes().len() / 1024;

    println!(
        "Proving time: {proving_time:?}\n\
        Verifying time (composite proof): {verifying_time:?}\n\
        Compression time: {compression_time:?}\n\
        Verifying time (succinct proof): {succinct_verifying_time:?}\n\n\
        Composite proof size: {composite_size}kB, Succinct proof size: {succinct_size}kB",
    );
}
