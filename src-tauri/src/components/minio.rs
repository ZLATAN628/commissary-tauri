// minio 文件服务器相关操作
use chrono::Local;
use regex::Regex;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, Region};

pub async fn upload_file0(path: String, file_type: String) -> Result<String, S3Error> {
    // This requires a running minio server at localhost:9000
    let bucket = Bucket::new(
        "commissary-tauri",
        Region::Custom {
            region: "eu-central-1".to_owned(),
            endpoint: "http://172.16.140.83:9000".to_owned(),
        },
        Credentials::new(
            Some("JIgP2e5LurzI2pyz"),
            Some("ISmTnCo60z9ZrqLhvShBExxojGjXyKq6"),
            None,
            None,
            None,
        )?,
    )?
    .with_path_style();
    // 生成文件名
    let now = Local::now();
    let regexp = Regex::new("\\.(\\w+)$").unwrap();
    let mut suffix = String::new();
    for cap in regexp.captures_iter(&path) {
        suffix.push_str(&cap[1]);
    }
    let s3_path = format!("/image/{}/{}.{}", file_type, now.timestamp_millis(), suffix);

    let mut path = tokio::fs::File::open(path).await?;
    println!("path: {}", s3_path);
    bucket.put_object_stream(&mut path, &s3_path).await?;

    Ok(format!(
        "http://172.16.140.83:9000/commissary-tauri{}",
        &s3_path
    ))
}
