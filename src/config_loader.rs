use crate::config::connections::db_connection::get_db_connection;
use crate::config::connections::redis::get_redis_connection;
use crate::config::connections::s3_connection::get_s3_connection;
use crate::config::{Config, Connections};

const CONFIG_PATH: &str = "config.json";

pub async fn load_config() -> Connections {
    let file = std::fs::read_to_string(CONFIG_PATH).expect("Could not read config file");
    let config: Config = serde_json::from_str(&file).expect("Could not parse config file");
    let Config {
        postgres,
        redis,
        s3,
    } = config;
    let db = get_db_connection(postgres).await;
    let s3 = get_s3_connection(s3).await;
    let redis = get_redis_connection(redis);
    match (db, s3, redis) {
        (Ok(db), Ok(s3), Ok(redis)) => Connections { db, s3, redis },
        others => {
            let mut errors = vec![];
            if let Err(err) = others.0 {
                errors.push(format!("Database: {}", err));
            }
            if let Err(err) = others.1 {
                errors.push(format!("S3: {}", err));
            }
            if let Err(err) = others.2 {
                errors.push(format!("Redis: {}", err));
            }
            panic!("Could not establish connections: {}", errors.join(", "));
        }
    }
}
