use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

/// 32 byte hash
pub type Hash = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub address: [u8; 20],
    pub balance: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub from: [u8; 20],
    pub to: [u8; 20],
    pub value: u64,
    pub nonce: u64,
    pub signature: Option<[u8; 64]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub transactions_root: Hash,
    pub number: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn hash(&self) -> Hash {
        let bytes = bincode::serialize(&self.header).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&bytes);
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub accounts: HashMap<[u8; 20], Account>,
}

impl State {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn apply(&mut self, tx: &Transaction) -> Result<(), String> {
       let sender = self.accounts.get_mut(&tx.from).ok_or("Sender account not found")?;
        if sender.nonce != tx.nonce {
            return Err("Invalid nonce".to_string());
        }
        if sender.balance < tx.value {
            return Err("Insufficient balance".to_string());
        }

        let receiver = self.accounts.entry(tx.to).or_insert(Account {
            address: tx.to,
            balance: 0,
            nonce: 0,
        });

        sender.balance -= tx.value;
        sender.nonce += 1;
        receiver.balance += tx.value;

        Ok(())
    }

    pub fn root(&self) -> Hash {
        let bytes = bincode::serialize(&self.accounts).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&bytes);
        hasher.finalize().into()
    }
}