use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename = "database")]
pub struct DatabaseConfig {
    pub database_url: String,
    pub database_name: String,
    pub database_username: String,
    pub database_password: String,
}

impl DatabaseConfig {
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.database_username,
            self.database_password,
            self.database_url,
            self.database_name
        )
    }
}