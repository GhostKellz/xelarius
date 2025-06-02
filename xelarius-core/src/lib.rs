use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sled::Db;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Option<String>, // For signature validation stub
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{:?}{}",
            index, timestamp, transactions, &previous_hash
        ));
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }

    pub fn is_valid(&self, prev_block: &Block) -> bool {
        self.index == prev_block.index + 1
            && self.previous_hash == prev_block.hash
            && self.hash
                == Block::calculate_hash(
                    self.index,
                    self.timestamp,
                    &self.transactions,
                    &self.previous_hash,
                )
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: u64,
        transactions: &Vec<Transaction>,
        previous_hash: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{:?}{}",
            index, timestamp, transactions, previous_hash
        ));
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, 0, vec![], "0".into());
        Blockchain {
            chain: vec![genesis],
        }
    }

    pub fn latest_hash(&self) -> String {
        self.chain.last().unwrap().hash.clone()
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, timestamp: u64) -> bool {
        let index = self.chain.len() as u64;
        let previous_hash = self.latest_hash();
        let new_block = Block::new(index, timestamp, transactions, previous_hash.clone());
        let prev_block = self.chain.last().unwrap();
        if new_block.is_valid(prev_block) {
            self.chain.push(new_block);
            true
        } else {
            false
        }
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            if !self.chain[i].is_valid(&self.chain[i - 1]) {
                return false;
            }
        }
        true
    }
}

pub struct Mempool {
    pub txs: Arc<Mutex<Vec<Transaction>>>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            txs: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn add_tx(&self, tx: Transaction) {
        self.txs.lock().unwrap().push(tx);
    }
    pub fn drain(&self) -> Vec<Transaction> {
        let mut txs = self.txs.lock().unwrap();
        let drained = txs.clone();
        txs.clear();
        drained
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PersistentChain {
    pub db: Db,
}

impl Clone for PersistentChain {
    fn clone(&self) -> Self {
        PersistentChain {
            db: self.db.clone(),
        }
    }
}

impl PersistentChain {
    pub fn open(path: &str) -> sled::Result<Self> {
        let db = sled::open(path)?;
        Ok(PersistentChain { db })
    }
    pub fn store_block(&self, block: &Block) -> sled::Result<()> {
        let key = block.index.to_be_bytes();
        let value = bincode::serialize(block).unwrap();
        self.db.insert(key, value)?;
        Ok(())
    }
    pub fn get_block(&self, index: u64) -> Option<Block> {
        let key = index.to_be_bytes();
        self.db
            .get(key)
            .ok()
            .flatten()
            .and_then(|ivec| bincode::deserialize(&ivec).ok())
    }
}

pub struct StateStore {
    pub balances: HashMap<String, u64>,
    pub nonces: HashMap<String, u64>,
}

impl StateStore {
    pub fn new() -> Self {
        StateStore {
            balances: HashMap::new(),
            nonces: HashMap::new(),
        }
    }
    pub fn apply_tx(&mut self, tx: &Transaction) -> bool {
        // Dummy signature check
        if tx.signature.is_none() {
            return false;
        }
        // Nonce check
        let nonce = self.nonces.get(&tx.from).cloned().unwrap_or(0);
        if tx.nonce != nonce {
            return false;
        }
        // Balance check
        let bal = self.balances.get(&tx.from).cloned().unwrap_or(0);
        if bal < tx.amount {
            return false;
        }
        // Apply
        *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount;
        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        self.nonces.insert(tx.from.clone(), tx.nonce + 1);
        true
    }
}

impl Default for StateStore {
    fn default() -> Self {
        Self::new()
    }
}

// WASM contract engine using Wasmtime
pub struct WasmEngine {
    pub engine: wasmtime::Engine,
    pub store: wasmtime::Store<()>,
}

impl WasmEngine {
    pub fn new() -> Self {
        let engine = wasmtime::Engine::default();
        let store = wasmtime::Store::new(&engine, ());
        WasmEngine { engine, store }
    }

    pub fn execute(&mut self, code: &[u8], func: &str, _input: &[u8]) -> anyhow::Result<Vec<u8>> {
        use wasmtime::{Instance, Module};
        let module = Module::from_binary(&self.engine, code)?;
        let instance = Instance::new(&mut self.store, &module, &[])?;
        let func = instance
            .get_func(&mut self.store, func)
            .ok_or_else(|| anyhow::anyhow!("Function not found"))?;
        let typed = func.typed::<(i32, i32), i32>(&self.store)?;
        // For demo: pass dummy args, real ABI would marshal input/output
        let result = typed.call(&mut self.store, (0, 0))?;
        Ok(result.to_le_bytes().to_vec())
    }
}

impl Default for WasmEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blockchain_add_block() {
        let mut chain = Blockchain::new();
        let tx = Transaction {
            from: "a".into(),
            to: "b".into(),
            amount: 10,
            nonce: 1,
            signature: Some("sig".into()),
        };
        let ok = chain.add_block(vec![tx.clone()], 123);
        assert!(ok);
        assert_eq!(chain.chain.len(), 2);
        assert_eq!(chain.chain[1].transactions[0], tx);
    }

    #[test]
    fn test_chain_validation() {
        let mut chain = Blockchain::new();
        let tx1 = Transaction {
            from: "a".into(),
            to: "b".into(),
            amount: 10,
            nonce: 1,
            signature: Some("sig1".into()),
        };
        let tx2 = Transaction {
            from: "b".into(),
            to: "c".into(),
            amount: 5,
            nonce: 2,
            signature: Some("sig2".into()),
        };
        chain.add_block(vec![tx1], 1);
        chain.add_block(vec![tx2], 2);
        assert!(chain.is_valid_chain());
    }

    #[test]
    fn test_invalid_block() {
        let mut chain = Blockchain::new();
        let tx = Transaction {
            from: "a".into(),
            to: "b".into(),
            amount: 10,
            nonce: 1,
            signature: Some("sig".into()),
        };
        let mut block = Block::new(1, 123, vec![tx], "bad_hash".into());
        block.hash = "tampered".into();
        chain.chain.push(block);
        assert!(!chain.is_valid_chain());
    }
}
