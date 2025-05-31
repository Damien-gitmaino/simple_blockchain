mod block;
mod blockchain;
mod transaction;

use actix_web::{App, HttpServer, web, HttpResponse, Responder};
use blockchain::Blockchain;
use std::sync::Mutex;

struct AppState {
    blockchain: Mutex<Blockchain>,
}

async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(blockchain.get_chain())
}

async fn mine_block(data: web::Data<AppState>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    let result = blockchain.mine_block();
    HttpResponse::Ok().body(result)
}

async fn post_transaction(data: web::Data<AppState>, tx: web::Json<transaction::Transaction>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_transaction(tx.into_inner());
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
