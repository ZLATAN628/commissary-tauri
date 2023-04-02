use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::JsResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QqResponse {
    status: String,
    retcode: i32,
    msg: Option<String>,
    wording: Option<String>,
    data: Value,
    echo: Option<String>,
}

pub async fn test0() -> Result<String, Box<dyn Error>> {
    // let res = async_get("/get_login_info").await?;
    let param = json!({
        "user_id": 540188804,
        "message": "你好你一共消费 550 元 请立即缴费"
    });
    let res = async_post("send_private_msg", &param).await?;
    Ok(JsResult::success(res))
}

pub async fn async_get(api: &str) -> Result<QqResponse, Box<dyn Error>> {
    let body = reqwest::get(format!("http://127.0.0.1:5700/{}", api))
        .await
        .unwrap()
        .text()
        .await?;
    let res: QqResponse = serde_json::from_str(&body)?;
    println!("{:?}", res);
    Ok(res)
}

pub async fn async_post(api: &str, param: &Value) -> Result<QqResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let body = client
        .post(format!("http://127.0.0.1:5700/{}", api))
        .json(param)
        .send()
        .await?
        .text()
        .await?;

    let res: QqResponse = serde_json::from_str(&body)?;
    println!("{:?}", res);
    Ok(res)
}
