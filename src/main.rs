use axum::{Router, routing::get};
use std::net::SocketAddr;

mod users;
use users::routes as user_routes;


async fn hello_orugu() -> &'static str {
    "Hello Orugu!"
}

fn main_router() -> Router{
    Router::new()
        .route("/" , get(hello_orugu))
        .merge(user_routes::users_router())
}

#[tokio::main]
async fn main() {
    let app = main_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Orugu Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}