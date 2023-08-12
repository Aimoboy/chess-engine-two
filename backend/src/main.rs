use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ExampleArgs {
    a: i32,
    b: i32
}

#[derive(Deserialize, Serialize)]
enum RpcMethod {
    Example(ExampleArgs)
}

#[derive(Deserialize, Serialize)]
struct RpcRequest {
    method: RpcMethod
}

#[derive(Deserialize, Serialize)]
struct RpcResponse {
    result: i32
}


async fn rpc_handler(payload: web::Json<RpcRequest>) -> impl Responder {

    // Handle RPC requests here
    match &payload.method {
        RpcMethod::Example(args) => {
            // Code here
        }
    }

    let response = RpcResponse {
        result: 10
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let test = RpcRequest {
        method: RpcMethod::Example( ExampleArgs { a: 1, b: 2 })
    };
    println!("{}", serde_json::to_string(&test)?);

    HttpServer::new(|| {
        App::new().route("/rpc", web::post().to(rpc_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// curl -X POST http://127.0.0.1:8080/rpc -H "Content-Type: application/json" -d 100
