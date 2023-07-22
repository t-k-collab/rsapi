use axum::{
    // http::StatusCode,
    // response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
};
use dotenv::dotenv;
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("{:?}", env::var("ADDRESS").unwrap_or("".to_string()));
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json", get(|| async { Json(json!({ "data": 42 })) }));

    let address = env::var("ADDRESS").unwrap_or("".to_string());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("cannot serve"));
}
