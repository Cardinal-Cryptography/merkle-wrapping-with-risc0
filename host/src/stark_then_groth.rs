use risc0_groth16::docker::stark_to_snark;
use risc0_zkvm::{get_prover_server, recursion::identity_p254, CompactReceipt, ExecutorEnv, ExecutorImpl, InnerReceipt, ProverOpts, Receipt, VerifierContext, default_prover};
use methods::{MERKLE_ELF, MERKLE_ID};

pub fn wrapping_pipeline(env: ExecutorEnv) {
    // let receipt = prover.prove_session(&ctx, &session).unwrap();
    // let claim = receipt.get_claim().unwrap();
    // let composite_receipt = receipt.inner.composite().unwrap();
    // let succinct_receipt = prover.compress(composite_receipt).unwrap();
    // let journal = session.journal.unwrap().bytes;

    // let ident_receipt = identity_p254(&succinct_receipt).unwrap();
    // let seal_bytes = ident_receipt.get_seal_bytes();

    let now = || std::time::Instant::now();
    let prover = default_prover();

    let (proving_time, receipt) = measure(|| prover
        .prove(env, MERKLE_ELF)
        .unwrap());

    let seal_bytes = receipt.inner.composite();

    let seal = stark_to_snark(&seal_bytes).unwrap().to_vec();

    let receipt = Receipt::new(
        InnerReceipt::Compact(CompactReceipt { seal, claim }),
        journal,
    );

    receipt.verify(MERKLE_ID).unwrap();
}
