use std::sync::Arc;
use sea_orm::DatabaseConnection;

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
}