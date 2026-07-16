use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub address: [u8; 20],
    pub balance: u64,
    pub nonce: u64,
}