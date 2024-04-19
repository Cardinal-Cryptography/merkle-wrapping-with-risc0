#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MerkleProof<Digest> {
    pub root: Digest,
    pub nodes: Vec<Digest>,
    pub leaf_index: u32,
}
