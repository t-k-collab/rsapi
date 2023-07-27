use dotenv::dotenv;
use std::env;

mod entities;
mod infrastructures;
mod interfaces;
mod use_cases;
use infrastructures::router::init_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = init_router();

    let address = env::var("ADDRESS").unwrap_or("".to_string());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("cannot serve"));
}
