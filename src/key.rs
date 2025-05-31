use secp256k1::ecdsa::Signature;
use secp256k1::{rand, PublicKey, SecretKey};
use secp256k1::{Secp256k1, Message};
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

pub fn generate_keypair() -> (secp256k1::SecretKey, secp256k1::PublicKey) {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    (secret_key, public_key)
}

fn hash_transaction(tx: &Transaction) -> [u8; 32] {
    let data = format!("{}|{}|{}", tx.get_sender(), tx.get_receiver(), tx.get_amount());
    let hash = Sha256::digest(data.as_bytes());
    hash.into()
}

pub fn sign_transaction(tx: &Transaction, secret_key: &SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let hash = hash_transaction(tx);
    let message = Message::from_digest_slice(&hash).expect("32 bytes");
    secp.sign_ecdsa(&message, secret_key)
}

pub fn verify_transaction_signature(
    tx: &Transaction,
    signature: &Signature,
    public_key: &PublicKey,
) -> bool {
    let secp = Secp256k1::new();
    let hash = hash_transaction(tx);
    let message = Message::from_digest_slice(&hash).expect("32 bytes");
    secp.verify_ecdsa(&message, signature, public_key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    fn mock_transaction() -> Transaction {
        Transaction::new("Alice".to_string(), "Bob".to_string(), 100.0)
    }

    #[test]
    fn test_generate_keypair_validity() {
        let (_secret_key, public_key) = generate_keypair();
        
        assert_eq!(public_key.serialize().len(), 33);
    }

    #[test]
    fn test_sign_and_verify_transaction_success() {
        let tx = mock_transaction();
        let (secret_key, public_key) = generate_keypair();
        let signature = sign_transaction(&tx, &secret_key);

        let is_valid = verify_transaction_signature(&tx, &signature, &public_key);
        assert!(is_valid);
    }

    #[test]
    fn test_verify_transaction_signature_failure_wrong_key() {
        let tx = mock_transaction();
        let (secret_key1, _) = generate_keypair();
        let (_, public_key2) = generate_keypair(); // Wrong key

        let signature = sign_transaction(&tx, &secret_key1);
        let is_valid = verify_transaction_signature(&tx, &signature, &public_key2);

        assert!(!is_valid);
    }

    #[test]
    fn test_verify_transaction_signature_failure_modified_transaction() {
        let original_tx = mock_transaction();
        let modified_tx = Transaction::new("Alice".to_string(), "Charlie".to_string(), 100.0);
        let (secret_key, public_key) = generate_keypair();

        let signature = sign_transaction(&original_tx, &secret_key);
        let is_valid = verify_transaction_signature(&modified_tx, &signature, &public_key);

        assert!(!is_valid);
    }

    #[test]
    fn test_hash_transaction_length() {
        let tx = mock_transaction();
        let hash = super::hash_transaction(&tx);
        assert_eq!(hash.len(), 32); // SHA-256 = 32 bytes
    }
}