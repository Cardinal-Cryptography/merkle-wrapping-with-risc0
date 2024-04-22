use risc0_groth16::docker::stark_to_snark;
use risc0_zkvm::{
    get_prover_server, recursion::identity_p254, CompactReceipt, ExecutorEnv, InnerReceipt,
    ProverOpts, Receipt,
};

use methods::{MERKLE_ELF, MERKLE_ID};

use crate::measure;

pub fn wrapping_pipeline(env: ExecutorEnv) {
    // ======== Prepare the prover =================================================================
    let prover = get_prover_server(&ProverOpts::default()).unwrap();

    // ======== Generate composite proof ===========================================================
    let (proving_time, receipt) = measure(|| prover.prove(env, MERKLE_ELF).unwrap());
    let composite_receipt = receipt.inner.composite().unwrap();

    // ======== Compress the receipt ===============================================================
    let (compression_time, succinct_receipt) =
        measure(|| prover.compress(composite_receipt).unwrap());

    // ======== Convert to a proof over Poseidon-254 ===============================================
    let (conversion_time, ident_receipt) = measure(|| identity_p254(&succinct_receipt).unwrap());
    let final_stark_seal = ident_receipt.get_seal_bytes();

    // ======== Convert to a proof over Groth16 ====================================================
    let (groth_proving_time, groth_seal) =
        measure(|| stark_to_snark(&final_stark_seal).unwrap().to_vec());
    let groth_receipt = Receipt::new(
        InnerReceipt::Compact(CompactReceipt {
            seal: groth_seal.clone(),
            claim: composite_receipt.get_claim().unwrap(),
        }),
        receipt.journal.bytes,
    );

    // ======== Verify the Groth16 proof ===========================================================
    let (groth_verifying_time, _) = measure(|| {
        groth_receipt.verify(MERKLE_ID).unwrap();
    });

    // ======== Report the times and sizes =========================================================
    let recursion_proof_size = final_stark_seal.len() / 1024;
    let groth_seal_size = groth_seal.len();

    println!(
        "Proving time (STARK): {proving_time:?}\n\
        Compression time (composite STARK -> succinct STARK): {compression_time:?}\n\
        Conversion time (succinct STARK -> recursion proof over Poseidon): {conversion_time:?}\n\
        Wrapping (recursion proof -> Groth16): {groth_proving_time:?}\n\
        Verifying time (Groth16): {groth_verifying_time:?}\n\n\
        Recursion proof size: {recursion_proof_size}kB, Groth16 proof size: {groth_seal_size:?} bytes",
    );
}
