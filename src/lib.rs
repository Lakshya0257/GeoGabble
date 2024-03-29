mod connection;
mod routes;
use axum::Router;
use routes::create_routes;
mod core;
mod models;
mod utils;
mod error;
mod logging;

pub async fn run() -> Router {
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = create_routes().await;
    app
    // axum::serve(listener, app).await.unwrap();
}
