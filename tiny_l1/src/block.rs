use crate::prelude::*;
use crate::transaction::*;
use crate::state::*;
use crate::merkle::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub tx_root: Hash,
    pub height: u64,
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
    pub fn new(parent: &Block, transactions: Vec<Transaction>, state: &State, timestamp: u64) -> Self {
        let tree = MerkleTree::build(&transactions);
        Block {
            header: BlockHeader {
                parent_hash: parent.hash(),
                state_root: state.root(),
                tx_root: tree.root(),
                height: parent.header.height + 1,
                timestamp,
            },
            transactions,
        }
    }

    pub fn genesis(state: &State, timestamp: u64) -> Self {
        Block {
            header: BlockHeader {
                parent_hash: [0u8; 32],
                state_root: state.root(),
                tx_root: [0u8; 32],
                height: 0,
                timestamp,
            },
            transactions: vec![],
        }
    }


}