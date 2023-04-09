use super::DB_POOL;
use mysql::prelude::*;
use std::fmt::Display;

///
/// 获取配置信息：
/// @config_type:
///     1-需要发送消息的群聊号
///
pub fn get_config<T>(config_type: i32, default_value: T) -> T
where
    T: FromValue + Copy + Display,
{
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    let value: Vec<T> = conn
        .query(format!(
            "select value from commissary_config where type = {}",
            config_type
        ))
        .unwrap();
    match value.get(0) {
        Some(&value) => value,
        None => default_value,
    }
}
