use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;


use sqlx::mysql::MySqlPoolOptions;

mod users;
mod handlers;
mod utils;
mod states;
mod settings;

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

    let pool = crate::settings::create_db_pool().await;
    let app = main_router()
        .with_state(Arc::new(AppState { db: pool.clone() }));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Orugu Server running on http://{},update: verify", addr);

    // axum::serve(listener, router).await.unwrap()
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(), //listener
        app, //router
    )
    .await
    .unwrap();
}