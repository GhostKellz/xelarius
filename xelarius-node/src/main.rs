use libp2p::{PeerId, identity, swarm::Swarm, gossipsub::{Gossipsub, GossipsubEvent, MessageAuthenticity, IdentTopic, GossipsubConfig}, tcp::TokioTcpConfig, noise, yamux, Transport, core::upgrade, Multiaddr};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;
use tokio::time::{Duration, sleep};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use xelarius_core::{Blockchain, Mempool, PersistentChain, Transaction};

#[tokio::main]
async fn main() {
    let chain = Arc::new(Mutex::new(Blockchain::new()));
    let mempool = Arc::new(Mempool::new());
    let db = PersistentChain::open("/tmp/xelarius_chain").expect("sled open");
    let state = Arc::new(Mutex::new(xelarius_core::StateStore::new()));

    // Authority key (for PoA, just a placeholder)
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // --- libp2p networking setup ---
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&id_keys).unwrap())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();
    let gossipsub_config = GossipsubConfig::default();
    let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(id_keys.clone()), gossipsub_config).unwrap();
    let topic = IdentTopic::new("xelarius-blocks");
    gossipsub.subscribe(&topic).unwrap();
    let mut swarm = Swarm::new(transport, gossipsub, peer_id);

    // Channel for sending new blocks/txs to the network
    let (net_tx, mut net_rx) = mpsc::unbounded_channel();

    // Spawn a task to handle libp2p events
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = net_rx.recv() => {
                    // Broadcast new block/tx
                    let _ = swarm.behaviour_mut().publish(topic.clone(), msg);
                }
                event = swarm.next() => {
                    if let Some(libp2p::swarm::SwarmEvent::Behaviour(GossipsubEvent::Message { message, .. })) = event {
                        // Handle incoming block/tx message
                        println!("Received gossipsub message: {:?}", message.data);
                        // TODO: Deserialize and process block/tx
                    }
                }
            }
        }
    });

    // JSON-RPC API stub
    let chain_rpc = chain.clone();
    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8545").await.unwrap();
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let chain_rpc = chain_rpc.clone();
            tokio::spawn(async move {
                let chain = chain_rpc.lock().unwrap();
                let resp = json!({"jsonrpc":"2.0","result":chain.chain.len(),"id":1});
                let _ = socket.write_all(resp.to_string().as_bytes()).await;
            });
        }
    });

    // Simulate consensus loop (PoA): produce a block every 5 seconds if mempool has txs
    let mempool_clone = mempool.clone();
    let db_clone = db.clone();
    let chain_consensus = chain.clone();
    let state_consensus = state.clone();
    let net_tx_consensus = net_tx.clone();
    tokio::spawn(async move {
        loop {
            let txs = mempool_clone.drain();
            if !txs.is_empty() {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                // Validate and apply txs
                let mut state = state_consensus.lock().unwrap();
                let valid_txs: Vec<_> = txs.into_iter().filter(|tx| state.apply_tx(tx)).collect();
                let mut chain = chain_consensus.lock().unwrap();
                chain.add_block(valid_txs.clone(), now);
                db_clone.store_block(&chain.chain.last().unwrap()).unwrap();
                println!("Block produced at {}", now);

                // Broadcast new block
                let serialized_block = serde_json::to_vec(&chain.chain.last().unwrap()).unwrap();
                net_tx_consensus.send(serialized_block).unwrap();
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
    let chain_print = chain.clone();
    tokio::spawn(async move {
        loop {
            let chain = chain_print.lock().unwrap();
            println!("Current chain: {:?}", chain.chain);
            sleep(Duration::from_secs(15)).await;
        }
    });

    // Keep main alive
    loop { sleep(Duration::from_secs(60)).await; }
}
