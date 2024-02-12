mod connection;
mod routes;
use routes::create_routes;
mod core;
mod models;
mod utils;

pub async fn run() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = create_routes().await;
    axum::serve(listener, app).await.unwrap();
}
