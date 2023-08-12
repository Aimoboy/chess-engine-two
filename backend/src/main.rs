use actix_web::{web, App, HttpServer};

mod rpc;
use crate::rpc::rpc_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/rpc", web::post().to(rpc_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// curl -X POST http://127.0.0.1:8080/rpc -H "Content-Type: application/json" -d "{\"method\":{\"Example\":{\"a\":1,\"b\":2}}}"
