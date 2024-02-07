use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct S3Config {
    pub bucket_name: String,
    pub region: String,
    pub endpoint: String,
    pub credentials: S3Credentials,
}

#[derive(Debug, Clone, Deserialize)]
pub struct S3Credentials {
    pub access_key: String,
    pub secret_key: String,
}
