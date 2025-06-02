// chain.rs
// Handles blockchain logic and wraps xelarius-core

use xelarius_core::{Blockchain, Mempool, PersistentChain, StateStore};

pub fn init_chain() -> std::sync::Arc<std::sync::Mutex<Blockchain>> {
    std::sync::Arc::new(std::sync::Mutex::new(Blockchain::new()))
}

pub fn init_mempool() -> std::sync::Arc<Mempool> {
    std::sync::Arc::new(Mempool::new())
}

pub fn init_db() -> PersistentChain {
    PersistentChain::open("/tmp/xelarius_chain").expect("sled open")
}

pub fn init_state() -> std::sync::Arc<std::sync::Mutex<StateStore>> {
    std::sync::Arc::new(std::sync::Mutex::new(StateStore::new()))
}
