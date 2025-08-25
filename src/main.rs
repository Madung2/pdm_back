use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;

use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;

mod users;
mod handlers;
mod utils;
mod states;

use users::routes as user_routes;
use states::AppState;


async fn hello_orugu() -> &'static str {
    "Hello Orugu!,update: pool connection"
}

fn main_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/" , get(hello_orugu))
        .merge(user_routes::users_router())
}


#[tokio::main]
async fn main() {
    // logger
    tracing_subscriber::fmt()
        .with_env_filter("info")  // RUST_LOG=debug cargo run 하면 debug도 찍힘
        .init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", database_url);

    // mysql
    // let pool = MySqlPoolOptions::connect(&DATABASE_URL).await.unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap();
    println!("MySQL pool created Successfully");

    let app = main_router()
        .with_state(Arc::new(AppState { db: pool.clone() }));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🚀 Orugu Server running on http://{},update: verify", addr);

    // axum::serve(listener, router).await.unwrap()
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(), //listener
        app, //router
    )
    .await
    .unwrap();
}