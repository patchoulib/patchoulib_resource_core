use std::sync::Arc;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::region::Region;
use crate::utils::config_parser::S3Config;

pub async fn get_connection(config: S3Config)
    -> Result<Arc<Bucket>, S3Error> {
    let region = Region::Custom {
        region: config.region,
        endpoint: config.endpoint,
    };
    let credentials = Credentials::new(
        Some(config.credentials.access_key.as_str()),
        Some(config.credentials.secret_key.as_str()),
        None,
        None,
        None,
    )?;
    let bucket = Bucket::new(
        config.bucket_name.as_str(),
        region,
        credentials,
    )?;
    Ok(Arc::new(bucket))
}