use std::time::{SystemTime, UNIX_EPOCH};
use xelarius_core::{Blockchain, Transaction};

fn main() {
    let mut chain = Blockchain::new();

    // Add a block with a dummy transaction for test
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let tx = Transaction {
        from: "genesis".into(),
        to: "test".into(),
        amount: 0,
        nonce: 0,
    };
    chain.add_block(vec![tx], now);

    for block in chain.chain.iter() {
        println!("Block #{}: {:?}", block.index, block);
    }
}
