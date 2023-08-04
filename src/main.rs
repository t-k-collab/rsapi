use dotenv::dotenv;
use std::env;

use sqlx::{self, postgres::PgPoolOptions};
// use postgres::PgConnection;
// use sqlx;

mod entities;
mod infrastructures;
mod interfaces;
mod use_cases;
use infrastructures::router::init_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = init_router();

    let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());
    let pool_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    let pool = match pool_connection {
        Ok(pool) => pool,
        Err(err) => panic!("Connection failure: {:?}", err),
    };

    let row = sqlx::query("SELECT * FROM families").execute(&pool).await;
    println!("{:?}", row);

    let address = env::var("ADDRESS").unwrap_or("".to_string());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("cannot serve"));
}
