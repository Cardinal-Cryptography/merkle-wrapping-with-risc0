#![no_main]
#![no_std]

extern crate alloc;

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::Digest;
use merkle_proof::MerkleProof;

risc0_zkvm::guest::entry!(main);

fn main() {
    let _merkle_proof = env::read::<MerkleProof<Digest>>();
    env::commit(&true);
}
