use actix_web::{web, http, App, HttpServer};
use actix_cors::Cors;

mod rpc;
use crate::rpc::rpc_handler;

pub mod chess {
    pub mod enums {
        pub mod chess_color;
        pub mod chess_piece;
    }
    pub mod constants;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
        .allowed_origin("http://localhost:4200")
        .allowed_methods(vec!["POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE);
        
        App::new()
        .wrap(cors)
        .route("/", web::post().to(rpc_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// curl -X POST http://127.0.0.1:8080/ -H "Content-Type: application/json" -d "{\"Example\":{\"a\":1,\"b\":2}}"
