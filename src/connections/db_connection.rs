use std::sync::Arc;
use sea_orm::{Database, DatabaseConnection};
use crate::utils::config_parser::db_config::DatabaseConfig;

pub async fn get_connection(db_config: DatabaseConfig)
    -> Result<Arc<DatabaseConnection>, sea_orm::DbErr>
{
    let db_url = db_config.get_database_url();
    match Database::connect(&db_url).await {
        Ok(db_conn) => Ok(Arc::new(db_conn)),
        Err(err) => Err(err),
    }
}