extern crate hex;
extern crate num_cpus;
extern crate rug;
extern crate sha2;

use std::i64;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use self::sha2::Digest;
use super::block::Block;
use self::rug::Integer;

pub struct ProofOfWork {
    block: Block,
    target: Integer,
}

// 挖矿难度
pub const BIT_LENGTH: u8 = 64;
pub const TARGET_BIT: i32 = 14;
pub const MAX_NOUNCE: i64 = i64::MAX;

impl ProofOfWork {
    pub fn new(b: &Block) -> ProofOfWork {
        let mut target: Integer = Integer::from_str_radix("1", 16).unwrap();
        target = target << (256 - TARGET_BIT);

        ProofOfWork {
            block: b.to_owned(),
            target: target,
        }
    }


    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut prev_block_hash = Vec::from(self.block.prev_block_hash.as_slice());
        let data = &self.block.data.clone();
        let timestamp = self.block.timestamp.to_string().into_bytes();
        let target: Vec<u8> = self.target.to_string_radix(16).as_bytes().to_vec();
        let _nonce = nonce.to_string().into_bytes();

        prev_block_hash.extend_from_slice(data.as_slice());
        prev_block_hash.extend_from_slice(timestamp.as_slice());
        prev_block_hash.extend_from_slice(target.as_slice());
        prev_block_hash.extend_from_slice(_nonce.as_slice());

        return prev_block_hash;
    }

    pub fn run(&self) -> (i64, Vec<u8>) {
        let mut hash: Vec<u8> = vec![];
        let mut nonce: i64 = 0;

        while nonce < MAX_NOUNCE {
            let mut hasher = sha2::Sha256::default();
            let data: Vec<u8> = self.prepare_data(nonce);
            hasher.input(&data);
            let result = hasher.result();

            if self.target
                .gt(&Integer::from_str_radix(&hex::encode(result), 16).unwrap())
            {
                hash.append(&mut result.as_slice().to_vec());
                break;
            }
            hash.clear();
            nonce = nonce + 1;
        }

        (nonce, hash.to_vec())
    }

}
