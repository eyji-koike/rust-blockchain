use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub from: [u8; 20],
    pub to: [u8; 20],
    pub value: u64,
    pub nonce: u64,
    #[serde(with = "serde_bytes")]
    pub signature: Option<Vec<u8>>, 
}

impl Transaction {
    pub fn new(from: [u8; 20], to: [u8; 20], value: u64, nonce: u64) -> Self {
        Self {
            from,
            to,
            value,
            nonce,
            signature: None,
        }
    }
}


