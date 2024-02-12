use crate::models::message_model::LocationDto;
use crate::utils::app_state::AppState;
use bb8_redis::{bb8::PooledConnection, RedisConnectionManager};
use redis::geo::{Coord, RadiusOptions, RadiusOrder, RadiusSearchResult, Unit};
use redis::AsyncCommands;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub async fn get_connected_users(client: AppState, user_id: String) -> HashSet<String> {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let users: HashSet<String> = pool
        .smembers(format!("connected:{}", user_id))
        .await
        .unwrap();
    users
}
pub async fn update_location(client: AppState, country: String, state: String, user_id: String) {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let val: Result<HashMap<String, String>, redis::RedisError> =
        pool.hgetall(format!("users:{}", user_id)).await;
    match val {
        Ok(value) => {
            if value.is_empty() {
                println!("Inserting value");
                user_add_update(&mut pool, country, state, user_id).await;
            } else {
                if value.get("country").unwrap() != &country
                    || value.get("state").unwrap() != &state
                {
                    location_changed(
                        &mut pool,
                        value.get("state").unwrap().to_owned(),
                        value.get("country").unwrap().to_owned(),
                        user_id.clone(),
                    )
                    .await;
                    user_add_update(&mut pool, country, state, user_id).await;
                } else {
                }
            }
        }
        Err(err) => {
            println!("E getting users:user_id: {:?}", err);
        }
    }
}

pub async fn check_buffer(client: AppState, user_id: String) -> bool {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let value: Result<Option<u64>, redis::RedisError> = pool.hget("buffer_states", &user_id).await;
    match value {
        Ok(value) => {
            if let Some(state) = value {
                // if state
                if state
                    < SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                {
                    println!("Buffer elapsed");
                    let now = SystemTime::now();
                    let up = now.checked_add(Duration::from_secs(5)).unwrap();
                    let _: () = pool
                        .hset(
                            "buffer_states",
                            &user_id,
                            up.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                        )
                        .await
                        .unwrap();
                    return false;
                } else {
                    println!("Buffer active");
                    return true;
                }
            } else {
                let now = SystemTime::now();
                let up = now.checked_add(Duration::from_secs(5)).unwrap();
                let _: () = pool
                    .hset(
                        "buffer_states",
                        &user_id,
                        up.duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                    )
                    .await
                    .unwrap();
                eprintln!("User {} not found", user_id);
                return false;
            }
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error: {:?}", err);
            return false;
        }
    }
}

pub async fn update_lat_lng(client: AppState, message: LocationDto) {
    let mut pool: PooledConnection<RedisConnectionManager> = client.redis.get().await.unwrap();
    let _: () = pool
        .geo_add(
            format!("curLoc:{}:{}", message.country, message.state),
            (
                Coord::lon_lat(message.longitude, message.latitude),
                &message.user_id,
            ),
        )
        .await
        .unwrap();

    let opts = RadiusOptions::default().with_dist().order(RadiusOrder::Asc);
    let ids: Vec<RadiusSearchResult> = pool
        .geo_radius(
            format!("curLoc:{}:{}", message.country, message.state),
            15.80,
            37.21,
            51.39,
            Unit::Kilometers,
            opts,
        )
        .await
        .unwrap();

    for id in ids.iter() {
        if &id.name != &message.user_id {
            let _: () = pool
                .sadd(format!("connected:{}", &message.user_id), &id.name)
                .await
                .unwrap();
        }
    }
}

pub async fn location_changed(
    pool: &mut PooledConnection<'_, RedisConnectionManager>,
    old_state: String,
    old_country: String,
    user_id: String,
) {
    let _: () = pool
        .zrem(format!("curLoc:{}:{}", old_country, old_state), user_id)
        .await
        .unwrap();
}

pub async fn user_add_update(
    pool: &mut PooledConnection<'_, RedisConnectionManager>,
    country: String,
    state: String,
    user_id: String,
) {
    let mut map = HashMap::new();
    map.insert("country".to_owned(), country);
    map.insert("state".to_owned(), state);
    let fields: Vec<(&String, &String)> = map.iter().map(|(k, v)| (k, v)).collect();
    let _: String = pool
        .hset_multiple(format!("users:{}", user_id), &fields)
        .await
        .unwrap();
    let _: () = pool
        .expire(format!("users:{}", user_id), 300)
        .await
        .unwrap();
}
