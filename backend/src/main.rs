use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ExampleArgs {
    a: i32,
    b: i32
}

#[derive(Deserialize, Serialize)]
struct ExampleResultArgs {
    res: i32
}

#[derive(Deserialize, Serialize)]
enum RpcMethod {
    Example(ExampleArgs)
}

#[derive(Deserialize, Serialize)]
enum RpcResult {
    ExampleResult(ExampleResultArgs)
}

#[derive(Deserialize, Serialize)]
struct RpcRequest {
    method: RpcMethod
}

#[derive(Deserialize, Serialize)]
struct RpcResponse {
    result: RpcResult
}


async fn rpc_handler(payload: web::Json<RpcRequest>) -> impl Responder {

    // Handle RPC requests here
    match &payload.method {
        RpcMethod::Example(_args) => {
            // Code here
        }
    }

    let response = RpcResponse {
        result: RpcResult::ExampleResult(ExampleResultArgs { res: 10 })
    };

    HttpResponse::Ok().json(response)
}

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
