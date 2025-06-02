use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
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
        };
        let tx2 = Transaction {
            from: "b".into(),
            to: "c".into(),
            amount: 5,
            nonce: 2,
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
        };
        let mut block = Block::new(1, 123, vec![tx], "bad_hash".into());
        block.hash = "tampered".into();
        chain.chain.push(block);
        assert!(!chain.is_valid_chain());
    }
}
