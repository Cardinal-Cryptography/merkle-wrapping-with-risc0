#![allow(dead_code)]

use risc0_zkvm::sha::{Digest, Impl, Sha256};
use merkle_proof::MerkleProof;

fn hash(left: Digest, right: Digest) -> Digest {
    *Impl::hash_bytes(&[left.as_bytes(), right.as_bytes()].concat())
}

pub fn get_short_merkle_proof() -> MerkleProof<Digest> {
    //             root
    //    h(0,0)            h(leaf, 0)
    // 0          0     leaf          0

    let leaf_value = Digest::from([1; 8]);              // index 6
    let parent_value = hash(leaf_value, Digest::ZERO);  // index 3
    let uncle_value = hash(Digest::ZERO, Digest::ZERO); // index 2
    let root_value = hash(uncle_value, parent_value);   // index 1

    MerkleProof {
        root: root_value,
        nodes: vec![leaf_value, Digest::ZERO, uncle_value],
        leaf_index: 6,
    }
}

pub fn get_long_merkle_proof() -> MerkleProof<Digest> {
    let leaf_value = Digest::from([1; 8]);
    let leaf_index = (1u64 << 34) + (1u64 << 33);

    let mut index = leaf_index;
    let nodes = [vec![leaf_value], Vec::from([Digest::ZERO; 34])].concat();

    let mut current_hash = leaf_value;
    while index > 1 {
        if index % 2 == 0 {
            current_hash = hash(current_hash, Digest::ZERO);
        } else {
            current_hash = hash(Digest::ZERO, current_hash);
        }
        index /= 2;
    }

    MerkleProof {
        root: current_hash,
        nodes,
        leaf_index,
    }
}
