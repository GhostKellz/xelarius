use std::time::{SystemTime, UNIX_EPOCH};
use xelarius_core::{Blockchain, Transaction, Mempool, PersistentChain};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use libp2p::{identity, PeerId};
use tokio::net::TcpListener;
use serde_json::json;

#[tokio::main]
async fn main() {
    let mut chain = Blockchain::new();
    let mempool = Arc::new(Mempool::new());
    let db = PersistentChain::open("/tmp/xelarius_chain").expect("sled open");
    let mut state = xelarius_core::StateStore::new();

    // Authority key (for PoA, just a placeholder)
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // Networking: libp2p stub (real implementation would go here)
    // TODO: Implement peer discovery, block/tx propagation, and chain sync

    // JSON-RPC API stub
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8545").await.unwrap();
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                // For demo: always return chain height
                let resp = json!({"jsonrpc":"2.0","result":chain.chain.len(),"id":1});
                let _ = socket.write_all(resp.to_string().as_bytes()).await;
            });
        }
    });

    // Simulate consensus loop (PoA): produce a block every 5 seconds if mempool has txs
    let mempool_clone = mempool.clone();
    let db_clone = db.clone();
    tokio::spawn(async move {
        loop {
            let txs = mempool_clone.drain();
            if !txs.is_empty() {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                // Validate and apply txs
                let valid_txs: Vec<_> = txs.into_iter().filter(|tx| state.apply_tx(tx)).collect();
                chain.add_block(valid_txs.clone(), now);
                db_clone.store_block(chain.chain.last().unwrap()).unwrap();
                println!("Block produced at {}", now);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    // For demo: add a dummy tx to mempool every 10 seconds
    let mempool_clone2 = mempool.clone();
    tokio::spawn(async move {
        let mut nonce = 0;
        loop {
            let tx = Transaction {
                from: "genesis".into(),
                to: "test".into(),
                amount: 1,
                nonce,
                signature: Some("dummy_sig".into()),
            };
            mempool_clone2.add_tx(tx);
            nonce += 1;
            sleep(Duration::from_secs(10)).await;
        }
    });

    // Print chain state every 15 seconds
    loop {
        println!("Current chain: {:?}", chain.chain);
        sleep(Duration::from_secs(15)).await;
    }
}
