extern crate time;
extern crate sha2;

use self::sha2::Digest;

#[derive(Clone)]
#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: Vec<u8>,
    pub prevBlockHash: Vec<u8>
}

pub struct BlockBuilder {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub hash: Vec<u8>,
    pub prevBlockHash: Vec<u8>,
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
            prevBlockHash: vec![],
            success: false
        }
    }

    pub fn data(&mut self, data: Vec<u8>) -> &mut BlockBuilder {
        self.data = data;
        self
    }

    pub fn prevBlockHash(&mut self, prevBlockHash: Vec<u8>) -> &mut BlockBuilder {
        self.prevBlockHash = prevBlockHash;
        self
    }

    pub fn hash(&mut self) -> &mut BlockBuilder {

        let mut hasher = sha2::Sha256::default();

        let mut data = self.data.clone();
        let mut timestamp = self.timestamp.to_string().into_bytes();
        let mut prevBlockHash = self.prevBlockHash.clone();

        data.append(&mut timestamp);
        data.append(&mut prevBlockHash);
        hasher.input(&data);
        self.hash = hasher.result().as_slice().to_vec();

        self
    }

    pub fn finalize(&self) -> Block {
        Block {
            timestamp: self.timestamp,
            data: self.data.clone(),
            hash: self.hash.clone(),
            prevBlockHash: self.prevBlockHash.clone()
        }
    }
}
