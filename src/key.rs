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
    let message = Message::from_slice(&hash).expect("32 bytes");
    secp.sign_ecdsa(&message, secret_key)
}

pub fn verify_transaction_signature(
    tx: &Transaction,
    signature: &Signature,
    public_key: &PublicKey,
) -> bool {
    let secp = Secp256k1::new();
    let hash = hash_transaction(tx);
    let message = Message::from_slice(&hash).expect("32 bytes");
    secp.verify_ecdsa(&message, signature, public_key).is_ok()
}