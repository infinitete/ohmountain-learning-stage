extern crate time;
extern crate sha2;

use std::vec::Vec;
use self::sha2::Digest;

#[derive(Clone)]
#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>
}

impl Block {

    pub fn hashing(&mut self) -> &mut Block {

        let mut hasher = sha2::Sha256::default();
        let mut data = Vec::from(self.data.as_slice());
        let timestamp = self.timestamp.to_string().into_bytes();

        data.extend_from_slice(&self.prev_block_hash.as_slice());
        data.extend_from_slice(timestamp.as_slice());

        hasher.input(&data);
        self.hash = hasher.result().as_slice().to_vec();

        self
    }
}

pub struct BlockBuilder {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub success: bool
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
            success: false
        }
    }

    pub fn data(&mut self, data: Vec<u8>) -> &mut BlockBuilder {
        self.data = data;
        self
    }

    pub fn finalize(&mut self) -> Block {
        Block {
            timestamp: self.timestamp,
            data: Vec::from(self.data.as_slice()),
            hash: vec![],
            prev_block_hash: vec![]
        }
    }
}
