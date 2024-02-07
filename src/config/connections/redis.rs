use std::sync::Arc;
use redis::{Connection, RedisResult};
use crate::config::RedisConfig;

pub fn get_redis_connection(config: RedisConfig) ->
RedisResult<Arc<Connection>> {
    let RedisConfig {
        host,
        port,
        password,
        db,
        username
    } = config;
    let url = match (username, password) {
        (Some(username), None) => format!("redis://{}@{}:{}", username, host, port),
        (None, Some(password)) => format!("redis://:{}@{}:{}", password, host, port),
        (Some(username), Some(password)) =>
            format!("redis://{}:{}@{}:{}", username, password, host, port),
        (None, None) => format!("redis://{}:{}", host, port),
    };
    let url = match db {
        Some(db) => format!("{}/{}", url, db),
        None => url,
    };
    let client = redis::Client::open(url)?;
    let con = client.get_connection();
    match con {
        Ok(con) => Ok(Arc::new(con)),
        Err(err) => Err(err),
    }
}
