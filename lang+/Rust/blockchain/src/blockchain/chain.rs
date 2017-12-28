use std::vec::Vec;
use super::block::Block;
use super::block::BlockBuilder;

#[derive(Debug)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain { blocks: vec![] }
    }

    pub fn the_creation(&mut self, data: Vec<u8>) {
        if self.blocks.len() == 0 {
            let mut builder = BlockBuilder::new();
            let mut block = builder.data(data).finalize();
            self.blocks.push(block.hashing().to_owned());
        }
    }

    pub fn append_block(&mut self, block: &mut Block) {
        if self.blocks.len() == 0 {
            self.the_creation(String::from("The Creation").as_bytes().to_vec());
            return;
        }

        let last_index = self.blocks.len() - 1;
        block.prev_block_hash = Vec::from(self.blocks[last_index].hash.as_slice());

        self.blocks.push(block.hashing().to_owned());
    }

    pub fn the_creation_block(&self) -> Option<Block> {
        match self.blocks.len() {
            0 => None,
            _ => Some(self.blocks[0].to_owned()),
        }
    }

    pub fn last_block(&self) -> Option<Block> {
        match self.blocks.len() {
            0 => None,
            n => Some(self.blocks[n - 1].to_owned()),
        }
    }
}
