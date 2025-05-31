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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn sample_transaction() -> Transaction {
        Transaction::new("Alice".to_string(), "Bob".to_string(), 42.0)
    }

    #[test]
    fn test_transaction_new_and_getters() {
        let tx = sample_transaction();
        assert_eq!(tx.get_sender(), "Alice");
        assert_eq!(tx.get_receiver(), "Bob");
        assert_eq!(tx.get_amount(), 42.0);
    }

    #[test]
    fn test_transaction_to_string_format() {
        let tx = sample_transaction();
        let expected = "Transaction { sender: Alice, receiver: Bob, amount: 42 }";
        assert_eq!(tx.to_string(), expected);
    }

    #[test]
    fn test_transaction_serialization() {
        let tx = sample_transaction();
        let serialized = serde_json::to_string(&tx).expect("Failed to serialize");
        assert!(serialized.contains("Alice"));
        assert!(serialized.contains("Bob"));
        assert!(serialized.contains("42.0"));
    }

    #[test]
    fn test_transaction_deserialization() {
        let json = r#"{
            "sender": "Charlie",
            "receiver": "Dana",
            "amount": 100.5
        }"#;
        let tx: Transaction = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(tx.get_sender(), "Charlie");
        assert_eq!(tx.get_receiver(), "Dana");
        assert_eq!(tx.get_amount(), 100.5);
    }

    #[test]
    fn test_signed_transaction_serialization_deserialization() {
        let tx = sample_transaction();
        let signed = SignedTransaction {
            transaction: tx.clone(),
            public_key: "abc123".to_string(),
            signature: "deadbeef".to_string(),
        };

        let serialized = serde_json::to_string(&signed).expect("Serialization failed");
        let deserialized: SignedTransaction = serde_json::from_str(&serialized).expect("Deserialization failed");

        assert_eq!(deserialized.transaction.get_sender(), tx.get_sender());
        assert_eq!(deserialized.public_key, "abc123");
        assert_eq!(deserialized.signature, "deadbeef");
    }
}