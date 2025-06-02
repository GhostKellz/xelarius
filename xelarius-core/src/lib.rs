use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u64, data: String, previous_hash: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, &previous_hash));
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

#[derive(Debug, Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, 0, "Genesis".into(), "0".into());
        Blockchain {
            chain: vec![genesis],
        }
    }

    pub fn latest_hash(&self) -> String {
        self.chain.last().unwrap().hash.clone()
    }

    pub fn add_block(&mut self, data: String, timestamp: u64) {
        let index = self.chain.len() as u64;
        let previous_hash = self.latest_hash();
        let new_block = Block::new(index, timestamp, data, previous_hash);
        self.chain.push(new_block);
    }
}
