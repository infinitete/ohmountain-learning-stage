extern crate blockchain;

use blockchain::blockchain::block;
use blockchain::blockchain::chain;

fn main() {

    let mut chain = chain::Chain::new();

    // 创世块
    chain.TheCreation(String::from("The Creation Block").as_bytes().to_vec());

    // 第二块
    chain.pushBlock(block::BlockBuilder::new().data(String::from("Hello World").as_bytes().to_vec()).hash().finalize());

    // 第三块
    chain.pushBlock(block::BlockBuilder::new().data(String::from("你好世界").as_bytes().to_vec()).hash().finalize());

    println!("{:?}", chain);
}
