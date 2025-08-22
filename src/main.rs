use axum::{Router, routing::get};
use std::net::SocketAddr;

async fn hello_orugu() -> &'static str {
    "Hello Orugu!"
}

fn init_router() -> Router{
    Router::new().route("/" , get(hello_orugu))
}


#[tokio::main]
async fn main() {
    let app = init_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Orugu Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}