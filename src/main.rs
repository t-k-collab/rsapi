use dotenv::dotenv;
use std::env;

mod entities;
mod infrastructures;
mod interfaces;
mod use_cases;
use crate::infrastructures::dbs::psql::connection::create_pool_connection;
use infrastructures::router::init_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());
    let pool = create_pool_connection(database_url).await;

    // let app = init_router();
    let app = init_router(pool);

    // let row = sqlx::query("SELECT * FROM families").execute(&pool).await;
    // println!("{:?}", row);

    let address = env::var("ADDRESS").unwrap_or("".to_string());
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("cannot serve"));
}
