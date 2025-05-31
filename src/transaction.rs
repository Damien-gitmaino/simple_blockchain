use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub public_key: String,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }

    pub fn get_sender(&self) -> &String {
        &self.sender
    }

    pub fn get_receiver(&self) -> &String {
        &self.receiver
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn to_string(&self) -> String {
        format!(
            "Transaction {{ sender: {}, receiver: {}, amount: {} }}",
            self.sender, self.receiver, self.amount
        )
    }
}