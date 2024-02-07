use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod hash;
mod query;
mod setter;

pub struct ResourceMapping {}

impl ResourceMapping {
    pub fn hash(bytes: &[u8]) -> String {
        todo!()
    }

    pub fn get(db_connection: Arc<DatabaseConnection>, hash: String) -> String {
        todo!()
    }

    pub fn set(
        db_connection: Arc<DatabaseConnection>,
        hash: String,
        value: String,
    ) -> Result<(), String> {
        todo!()
    }
}
