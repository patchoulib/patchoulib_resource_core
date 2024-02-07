pub mod connections;
pub mod parser;

use std::sync::Arc;
use redis::Connection;
use s3::Bucket;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
pub use parser::db_config::DatabaseConfig;
pub use parser::s3_config::S3Config;
pub use parser::redis_config::RedisConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub postgres: DatabaseConfig,
    pub s3: S3Config,
    pub redis: RedisConfig,
}

pub struct Connections {
    pub db: Arc<DatabaseConnection>,
    pub s3: Arc<Bucket>,
    pub redis: Arc<Connection>
}