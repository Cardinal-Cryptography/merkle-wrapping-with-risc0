#![no_main]
#![no_std]

extern crate alloc;

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Digest, Impl, Sha256};
use merkle_proof::MerkleProof;

risc0_zkvm::guest::entry!(main);

fn main() {
    fn hash(left: Digest, right: Digest) -> Digest {
        *Impl::hash_bytes(&[left.as_bytes(), right.as_bytes()].concat())
    }

    let MerkleProof { root, nodes, leaf_index } = env::read::<MerkleProof<Digest>>();

    let mut current_node_index = leaf_index;
    let mut current_hash = nodes[0];
    let mut current_path_index = 1;

    loop {
        current_hash = match current_node_index % 2 {
            0 => hash(current_hash, nodes[current_path_index]),
            1 => hash(nodes[current_path_index], current_hash),
            _ => unreachable!(),
        };

        current_path_index += 1;
        if current_path_index == nodes.len() {
            break;
        }
        current_node_index /= 2;
    }

    env::commit(&(current_hash == root));
}
