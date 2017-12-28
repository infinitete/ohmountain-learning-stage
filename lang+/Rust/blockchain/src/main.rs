extern crate blockchain;
extern crate hex;
extern crate rug;

use blockchain::blockchain::block;
use blockchain::blockchain::chain;

fn main() {
    let mut chain = chain::Chain::new();

    // 创世块
    chain.the_creation(String::from("The Creation Block").as_bytes().to_vec());

    // 第二块
    chain.append_block(&mut block::BlockBuilder::new()
        .data(String::from("Hello World").as_bytes().to_vec())
        .finalize());

    // 第三块
    chain.append_block(&mut block::BlockBuilder::new()
        .data(String::from("你好世界").as_bytes().to_vec())
        .finalize());

    for b in chain.blocks {
        println!("{} {}", b.get_hash_string(), b.nonce);
    }
}
