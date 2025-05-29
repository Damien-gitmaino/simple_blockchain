mod block;
mod blockchain;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use blockchain::Blockchain;
use serde::Deserialize;
use std::sync::Mutex;

struct AppState {
    blockchain: Mutex<Blockchain>,
}

#[derive(Deserialize)]
struct MineData {
    data: String,
}

async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(blockchain.get_chain())
}

async fn mine_block(info: web::Json<MineData>, data: web::Data<AppState>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_block(info.data.clone());
    HttpResponse::Ok().body("Block added successfully!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::new(4)),
    });

    println!("🚀 Running server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(blockchain.clone())
            .route("/blocks", web::get().to(get_blocks))
            .route("/mine", web::post().to(mine_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}