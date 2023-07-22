use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    println!("{:?}", env::var("ADDRESS").unwrap_or("".to_string()));
    println!("Hello, world!");
}
