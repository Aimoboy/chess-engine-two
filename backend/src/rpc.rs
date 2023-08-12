use actix_web::{web, HttpResponse, Responder};
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
pub struct RpcRequest {
    method: RpcMethod
}

#[derive(Deserialize, Serialize)]
struct RpcResponse {
    result: RpcResult
}


pub async fn rpc_handler(payload: web::Json<RpcRequest>) -> impl Responder {

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
