use super::block::Block;
use super::block::BlockBuilder;

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {

    pub fn new() -> Chain {
        Chain { blocks: vec![] }
    }

    pub fn TheCreation(&mut self, data: Vec<u8>) {
        if self.blocks.len() == 0 {
            let mut builder = BlockBuilder::new();
            let block = builder.data(data).hash().finalize();
            self.blocks.push(block);
        }
    }

    pub fn pushBlock(&mut self, block: Block) {

        if self.blocks.len() == 0 {
            self.TheCreation(String::from("The Creation").as_bytes().to_vec());
            return;
        }

       let mut b = block.clone();

        let lastIndex = self.blocks.len() - 1;
        b.prevBlockHash = self.blocks[lastIndex].hash.clone();

        self.blocks.push(b);
    }
}
