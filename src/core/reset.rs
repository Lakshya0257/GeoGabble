use std::collections::HashMap;

use bb8_redis::{bb8::PooledConnection, RedisConnectionManager};
use redis::{AsyncCommands, RedisError};

use crate::utils::app_state::AppState;

use super::location::location_changed;

pub async fn reset_user(user_id: String, client: AppState) {
    {
        client.connections.lock().await.remove(&user_id);
    }

    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let val: Result<HashMap<String, String>, redis::RedisError> =
        pool.hgetall(format!("users:{}", user_id)).await;
    match val {
        Ok(value) => {
            tokio::spawn(location_changed(
                client.clone(),
                value.get("state").unwrap().to_owned(),
                value.get("country").unwrap().to_owned(),
                user_id.clone(),
            ));
            tokio::spawn(drop_connected(client.clone(), user_id.clone()));
            tokio::spawn(drop_loc(client.clone(), user_id.clone()));
            tokio::spawn(drop_buffer(client.clone(), user_id.clone()));

            // let _  =location_changed(client.clone(), value.get("state").unwrap().to_owned(),
            // value.get("country").unwrap().to_owned(),
            // user_id.clone()).await;
            // let _ : Result<(), redis::RedisError>=pool.del(format!("connected:{}",&user_id)).await;
            // let _ : Result<(), redis::RedisError>=pool.del(format!("users:{}", &user_id)).await;
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub async fn drop_connected(client: AppState, user_id: String) -> Result<(), RedisError> {
    let mut pool = client.redis.get().await.unwrap();
    let y: Result<(), RedisError> = pool.del(format!("connected:{}", &user_id)).await;
    y
}
pub async fn drop_buffer(client: AppState, user_id: String) -> Result<(), RedisError> {
    let mut pool = client.redis.get().await.unwrap();
    let y: Result<(), RedisError> = pool.hdel("buffer_states", &user_id).await;
    y
}
pub async fn drop_loc(client: AppState, user_id: String) -> Result<(), RedisError> {
    let mut pool = client.redis.get().await.unwrap();
    let y: Result<(), RedisError> = pool.del(format!("users:{}", &user_id)).await;
    y
}
