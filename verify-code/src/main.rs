pub mod add_code;
pub mod dao;
pub mod model;

#[macro_use]
extern crate r_base;

use axum::extract::Query;
use axum::routing::MethodRouter;
use axum::{response::Html, routing::get, Router};
use mongodb::bson::doc;
use serde::Deserialize;
use std::net::SocketAddr;
// use serde_derive::Deserialize;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    r_base::init_modules(None).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/vcode", get(verify_code))
        .route("/add", get(add_code::add_code));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9999));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World from root ...."
}

async fn verify_code(Query(params): Query<Params>) -> String {
    let code = params.code.unwrap_or("no_code".to_string());
    let code = code.trim();
    //
    let b = dao::code_lib::exist(doc! {
     "_id":code
    })
    .await
    .unwrap_or(false);

    if b {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    code: Option<String>,
}
