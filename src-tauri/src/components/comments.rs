use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize};

use crate::JsResult;

use super::{ini_parse::parse_ini, DB_POOL};

#[derive(Debug, Serialize, Deserialize)]
struct Comments {
    stock_sn: i32,
    customer_name: String,
    state: i32,
}

pub fn add_comment0(state: i32, stock_sn: i32) -> String {
    let config = parse_ini();
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();

    match conn
        .query_map(
            format!(
                "select stock_sn, customer_name, state from commissary_product_comment where customer_name =  '{}' and stock_sn = {}",
                 &config.name, stock_sn
            ),
            |(stock_sn, customer_name, state)| Comments {
                stock_sn,
                customer_name,
                state,
            },
        )
        .unwrap()
        .pop()
    {
        Some(comment) => {
            match conn.exec_drop(
                "update commissary_product_comment set state = :state, update_time = sysdate()
                    where stock_sn = :stock_sn and customer_name = :customer_name",
                params! {
                    "stock_sn" => comment.stock_sn,
                    "customer_name" => &comment.customer_name,
                    "state" => state,
                },
            ) {
                Ok(_) => JsResult::success(String::from("修改成功")),
                Err(e) => JsResult::<String>::fail(format!("修改失败：{}", e)),
            }
        },
        None => {
            match conn.exec_drop(
                "insert into commissary_product_comment(stock_sn, customer_name, state, update_time)
                    values (:stock_sn, :customer_name, :state, sysdate())",
                params! {
                    "stock_sn" => stock_sn,
                    "customer_name" => &config.name,
                    "state" => state,
                },
            ) {
                Ok(_) => JsResult::success(String::from("新增成功")),
                Err(e) => JsResult::<String>::fail(format!("新增失败：{}", e)),
            }
        },
    }
}
