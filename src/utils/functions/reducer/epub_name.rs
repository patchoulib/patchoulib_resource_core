use sha1::{Digest, Sha1};

async fn get_epub_sha1(data: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hex = format!("{:x}", result);
    hex
}

async fn get_epub_name(data: &Vec<u8>, version: i64) -> String {
    let sha1 = get_epub_sha1(data).await;
    let epub_name = format!("{}-{}.epub",version ,sha1);
    epub_name
}