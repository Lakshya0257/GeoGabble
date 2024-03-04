use bb8_redis::{bb8::Pool, RedisConnectionManager};

pub async fn connect() -> Pool<RedisConnectionManager> {
    let redis_manager = RedisConnectionManager::new("redis://default:Sax7M3UbsenkvqS7ESwxLMMnE6zUNs20@redis-17581.c305.ap-south-1-1.ec2.cloud.redislabs.com:17581").unwrap();
    bb8_redis::bb8::Pool::builder()
        .build(redis_manager)
        .await
        .unwrap()
}
