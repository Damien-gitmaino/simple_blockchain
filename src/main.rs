mod block;
mod blockchain;
mod transaction;
mod key;
mod handlers;

use actix_web::{App, HttpServer, web};
use blockchain::Blockchain;
use std::sync::Mutex;
use crate::handlers::{get_blocks, get_keypair, mine_block, post_transaction, sign_tx};

struct AppState {
    blockchain: Mutex<Blockchain>,
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
