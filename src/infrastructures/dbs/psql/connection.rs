use sqlx::{self, postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_pool_connection(database_url: String) -> Pool<Postgres> {
    let pool_connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    return match pool_connection {
        Ok(pool) => pool,
        Err(err) => panic!("Connection failure: {:?}", err),
    };
    // return pool;
}
