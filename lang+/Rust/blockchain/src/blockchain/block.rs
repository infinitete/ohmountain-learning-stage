extern crate hex;
extern crate sha2;
extern crate time;

use std::vec::Vec;
use super::pow::ProofOfWork;

#[derive(Clone, Debug)]
pub struct Block {
    pub nonce: i64,
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
}

impl Block {
    pub fn hashing(&mut self) -> &mut Block {
        let (nonce, hash): (i64, Vec<u8>) = ProofOfWork::new(&self).run();

        self.hash = hash;
        self.nonce = nonce;

        self
    }

    pub fn get_pow(&self) -> ProofOfWork {
        ProofOfWork::new(&self)
    }

    pub fn get_hash_string(&self) -> String {
        hex::encode(&self.hash)
    }
}

pub struct BlockBuilder {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
}

impl BlockBuilder {
    pub fn new() -> BlockBuilder {
        let now = time::now();
        let sec = now.to_timespec().sec;

        BlockBuilder {
            timestamp: sec,
            data: vec![],
            hash: vec![],
            prev_block_hash: vec![],
        }
    }

    pub fn data(&mut self, data: Vec<u8>) -> &mut BlockBuilder {
        self.data = data;
        self
    }

    pub fn finalize(&mut self) -> Block {
        Block {
            nonce: 0,
            timestamp: self.timestamp,
            data: Vec::from(self.data.as_slice()),
            hash: vec![],
            prev_block_hash: vec![],
        }
    }
}
