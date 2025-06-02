use futures::StreamExt;
use libp2p::{
    core::upgrade,
    dns::TokioDnsConfig,
    gossipsub::{
        Behaviour as Gossipsub, Config as GossipsubConfig, Event as GossipsubEvent,
        IdentTopic, MessageAuthenticity,
    },
    identity,
    noise::{Keypair as NoiseKeypair, NoiseConfig, X25519Spec},
    swarm::SwarmBuilder,
    tcp::tokio::Transport,
    yamux::YamuxConfig,
    PeerId, Transport as Libp2pTransport,
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use xelarius_core::{Blockchain, Mempool, PersistentChain, Transaction, StateStore, Token};

mod network;
mod chain;
mod rpc;
mod tasks;

#[tokio::main]
async fn main() {
    // Setup core blockchain, mempool, persistent storage, state
    let chain = chain::init_chain();
    let mempool = chain::init_mempool();
    let db = chain::init_db();
    let state = chain::init_state();

    // Setup networking (libp2p, gossipsub, etc.)
    let (net_tx, net_rx, local_peer_id) = network::setup_network().await;
    println!("Local peer id: {:?}", local_peer_id);

    // Start JSON-RPC server
    rpc::start_rpc(chain.clone(), Arc::new(std::sync::Mutex::new(Token::new("Xelarius", "XZN", 42000000)))).await;

    // Start consensus, tx generation, and periodic printing tasks
    tasks::start_tasks(chain, mempool, db, state, net_tx, net_rx).await;
}
