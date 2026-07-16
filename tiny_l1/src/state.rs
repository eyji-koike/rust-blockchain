use crate::prelude::*;
use crate::transaction::*;
use crate::account::*;
use std::collections::HashMap;

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
        let sender = self
            .accounts
            .get(&tx.from)
            .ok_or("Sender account not found")?;
        if sender.nonce != tx.nonce {
            return Err("Invalid nonce".to_string());
        }
        if sender.balance < tx.value {
            return Err("Insufficient balance".to_string());
        }

        let sender_balance = sender.balance - tx.value;
        let sender_nonce = sender.nonce + 1;

        self.accounts.insert(
            tx.from,
            Account {
                address: tx.from,
                balance: sender_balance,
                nonce: sender_nonce,
            },
        );

        let receiver = self.accounts.entry(tx.to).or_insert(Account {
            address: tx.to,
            balance: 0,
            nonce: 0,
        });

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