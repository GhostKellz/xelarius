use osirius_core::Blockchain;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut chain = Blockchain::new();

    // Fake block for test
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    chain.add_block("Initial test block".into(), now);

    for block in chain.chain.iter() {
        println!("Block #{}: {:?}", block.index, block);
    }
}
