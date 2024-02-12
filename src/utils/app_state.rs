use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex as TMutex;

use axum::extract::{ws::Message, FromRef};
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use tokio::sync::mpsc;
// use crate::connection::connection::RedisPool;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub redis: Pool<RedisConnectionManager>,
    pub connections: Arc<TMutex<HashMap<String, mpsc::Sender<Message>>>>,
}
