use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Transaction {{ sender: {}, receiver: {}, amount: {} }}",
            self.sender, self.receiver, self.amount
        )
    }
}