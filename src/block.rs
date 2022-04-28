use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub previous_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
            &self.nonce,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        previous_block_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            previous_block_hash,
            nonce,
            payload,
        }
    }
}