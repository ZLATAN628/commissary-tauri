// minio 文件服务器相关操作

use std::fs::File;

use s3::creds::Credentials;
use s3::error::S3Error;
use s3::{Bucket, Region};

pub async fn upload_file0(path: String) -> Result<String, S3Error> {
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

    let s3_path = "/image";

    let mut path = tokio::fs::File::open(path).await?;
    let status_code = bucket.put_object_stream(&mut path, s3_path).await?;

    // bucket.put_object(s3_path, test).await.unwrap();

    // let response_data = bucket.get_object(s3_path).await?;
    // assert_eq!(response_data.status_code(), 200);

    // let response_data = bucket
    //     .get_object_range(s3_path, 100, Some(1000))
    //     .await
    //     .unwrap();
    // assert_eq!(response_data.status_code(), 206);
    // let (head_object_result, code) = bucket.head_object(s3_path).await?;
    // assert_eq!(code, 200);
    // assert_eq!(
    //     head_object_result.content_type.unwrap_or_default(),
    //     "application/octet-stream".to_owned()
    // );

    // let response_data = bucket.delete_object(s3_path).await?;
    // assert_eq!(response_data.status_code(), 204);
    Ok(String::from("上传成功"))
}
