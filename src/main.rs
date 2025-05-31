mod block;
mod blockchain;
mod transaction;
mod key;

use std::str::FromStr;
use actix_web::{App, HttpServer, web, HttpResponse, Responder};
use blockchain::Blockchain;
use std::sync::Mutex;
use secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};
use crate::key::{sign_transaction, verify_transaction_signature};
use crate::transaction::{SignedTransaction, Transaction};

struct AppState {
    blockchain: Mutex<Blockchain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TxSign { 
    tx: Transaction,
    private_key: String,
}

#[derive(Serialize)]
struct KeyPairResponse {
    secret_key: SecretKey,
    public_key: PublicKey,
}

async fn get_keypair() -> impl Responder {
    let (secret_key, public_key) = key::generate_keypair();
    let response = KeyPairResponse {
        secret_key,
        public_key,
    };

    HttpResponse::Ok().json(response)
}

async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(blockchain.get_chain())
}

async fn sign_tx(_data: web::Data<AppState>, sign: web::Json<TxSign>) -> impl Responder {
    let sec_key = SecretKey::from_str(&sign.private_key).unwrap();
    let signed_tx = sign_transaction(&sign.tx, &sec_key);
    HttpResponse::Ok().json(signed_tx)
}

async fn mine_block(data: web::Data<AppState>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    let result = blockchain.mine_block();
    HttpResponse::Ok().body(result)
}

async fn post_transaction(
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
    
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::new(4)),
    });

    println!("🚀 Blockchain server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/mine", web::get().to(mine_block))
            .route("/transaction", web::post().to(post_transaction))
            .route("/keypair", web::get().to(get_keypair))
            .route("/sign_tx", web::post().to(sign_tx))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
