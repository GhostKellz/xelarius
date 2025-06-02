// tasks.rs
// Handles consensus loop, tx generation, and periodic printing

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use xelarius_core::{Blockchain, Mempool, PersistentChain, StateStore, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, error};
use prometheus::{IntCounter, IntGauge, Registry};
use lazy_static::lazy_static;

lazy_static! {
    static ref BLOCK_PRODUCTION_RATE: IntCounter = IntCounter::new("block_production_rate", "Rate of block production").unwrap();
    static ref NETWORK_LATENCY: IntGauge = IntGauge::new("network_latency", "Network latency in ms").unwrap();
}

pub async fn start_tasks(
    chain: Arc<std::sync::Mutex<Blockchain>>,
    mempool: Arc<Mempool>,
    db: PersistentChain,
    state: Arc<std::sync::Mutex<StateStore>>,
    _net_tx: mpsc::UnboundedSender<Vec<u8>>,
    _net_rx: mpsc::UnboundedReceiver<Vec<u8>>,
) {
    let registry = Registry::new();
    registry.register(Box::new(BLOCK_PRODUCTION_RATE.clone())).unwrap();
    registry.register(Box::new(NETWORK_LATENCY.clone())).unwrap();

    // Consensus loop: produce a block every 5 seconds if mempool has txs
    tokio::spawn(async move {
        loop {
            let txs = mempool.drain();
            if !txs.is_empty() {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let valid_txs = {
                    let mut state = state.lock().unwrap();
                    txs.into_iter().filter(|tx| state.apply_tx(tx)).collect::<Vec<_>>()
                };
                let success = {
                    let mut chain = chain.lock().unwrap();
                    chain.add_block(valid_txs.clone(), now)
                };
                if success {
                    db.store_block(&chain.lock().unwrap().chain.last().unwrap()).unwrap();
                    BLOCK_PRODUCTION_RATE.inc();
                    info!("Block produced at {}", now);
                } else {
                    error!("Failed to produce block at {}", now);
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    // Dummy tx generator: add a tx every 10 seconds
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
            mempool.add_tx(tx.clone());
            nonce += 1;
            info!("Transaction added: {:?}", tx);
            sleep(Duration::from_secs(10)).await;
        }
    });

    // Print chain state every 15 seconds
    let chain_print = chain.clone();
    tokio::spawn(async move {
        loop {
            let chain_str = {
                let chain = chain_print.lock().unwrap();
                format!("{:?}", chain.chain)
            };
            info!("Current chain: {}", chain_str);
            sleep(Duration::from_secs(15)).await;
        }
    });

    // Keep main alive
    loop { sleep(Duration::from_secs(60)).await; }
}
