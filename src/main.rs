use axum::{
    // http::StatusCode,
    // response::IntoResponse,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::env;

mod infrastructures;

#[tokio::main]
async fn main() {
    dotenv().ok();

    use infrastructures::router;

    let app = Router::new()
        .route("/", get(router::health_check))
        .route("/members", post(router::create_member));

    let address = env::var("ADDRESS").unwrap_or("".to_string());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("cannot serve"));
}
