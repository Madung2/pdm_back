use dotenv::dotenv;
use std::env;
use sqlx::mysql::{MySqlPoolOptions, MySqlPool};

pub async fn create_db_pool() -> MySqlPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPool::connect(&database_url).await.unwrap()
}