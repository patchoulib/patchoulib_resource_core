use sea_orm::{Database, DatabaseConnection};
use crate::utils::config_parser::db_config::DatabaseConfig;

pub async fn get_connection(db_config: DatabaseConfig)
    -> Result<DatabaseConnection, sea_orm::DbErr>
{
    let db_url = db_config.get_database_url();
    Database::connect(&db_url).await
}