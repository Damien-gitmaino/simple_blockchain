use std::str::FromStr;
use actix_web::{web, HttpResponse, Responder};
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use crate::{key, AppState};
use crate::key::{sign_transaction, verify_transaction_signature};
use crate::transaction::{SignedTransaction, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxSign {
    tx: Transaction,
    private_key: String,
}

#[derive(Serialize)]
struct KeyPairResponse {
    secret_key: SecretKey,
    public_key: PublicKey,
}

pub async fn get_keypair() -> impl Responder {
    let (secret_key, public_key) = key::generate_keypair();
    let response = KeyPairResponse {
        secret_key,
        public_key,
    };

    HttpResponse::Ok().json(response)
}

pub async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(blockchain.get_chain())
}

pub async fn sign_tx(_data: web::Data<AppState>, sign: web::Json<TxSign>) -> impl Responder {
    let sec_key = SecretKey::from_str(&sign.private_key).unwrap();
    let signed_tx = sign_transaction(&sign.tx, &sec_key);
    HttpResponse::Ok().json(signed_tx)
}

pub async fn mine_block(data: web::Data<AppState>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    let result = blockchain.mine_block();
    HttpResponse::Ok().body(result)
}

pub async fn post_transaction(
    data: web::Data<AppState>,
    signed_tx: web::Json<SignedTransaction>,
) -> impl Responder {
    let signed_tx = signed_tx.into_inner();

    // Verify the transaction signature
    let public_key = secp256k1::PublicKey::from_str(&signed_tx.public_key).unwrap();
    let signature = secp256k1::ecdsa::Signature::from_str(&signed_tx.signature).unwrap();
    let is_valid = verify_transaction_signature(&signed_tx.transaction, &signature, &public_key);

    if !is_valid {
        return HttpResponse::Unauthorized().body("Invalid signature");
    }

    // Add transaction to mempool
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_transaction(signed_tx.transaction);

    HttpResponse::Ok().body("Transaction added to mempool")
}