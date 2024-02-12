use self::web_scoket::handler;
use crate::connection::connection::connect;
use crate::utils::app_state;
use axum::{http::Method, routing::get, Router};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex as TMutex;
mod sc_threads;
mod web_scoket;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    let r_client = connect().await;
    let app_state_redis = app_state::AppState {
        redis: r_client,
        connections: Arc::new(TMutex::new(HashMap::new())),
    };
    Router::new()
        .route("/location", get(handler))
        .layer(cors)
        .with_state(app_state_redis)
}
