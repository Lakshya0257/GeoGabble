use std::fmt::Result;

use redis::RedisError;

#[derive(Clone, Debug)]
pub struct RedisResponse {
    query: String,
    result: Result
}

trait DebugPrint {
    fn debug(&self);
}