use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ExampleArgs {
    a: i32,
    b: i32
}

#[derive(Deserialize, Serialize)]
pub struct ExampleResultArgs {
    res: i32
}

#[derive(Deserialize, Serialize)]
pub enum RpcRequest {
    Example(ExampleArgs)
}

#[derive(Deserialize, Serialize)]
pub enum RpcResponse {
    ExampleResult(ExampleResultArgs)
}

pub async fn rpc_handler(payload: web::Json<RpcRequest>) -> impl Responder {

    println!("test!");

    // Handle RPC requests here
    match &payload.0 {
        RpcRequest::Example(_args) => {
            // Code here
            println!("example!");
        }
    }

    let response = RpcResponse::ExampleResult(ExampleResultArgs { res: 10 });

    HttpResponse::Ok().json(response)
}
