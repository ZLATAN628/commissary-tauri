use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize};

use crate::JsResult;

use super::DB_POOL;

use super::{ini_parse::parse_ini, product::*};

#[derive(Deserialize, Serialize, Clone)]
pub struct PayRecord {
    pay_sn: Option<i32>,
    pay_ide: Option<i32>,
    stock_sn: Option<i32>,
    num: i32,
    customer_name: String,
    pay_time: Option<chrono::NaiveDateTime>,
    amount: f32,
}

impl PayRecord {
    fn from_settle_list(settle_list: Vec<Product>, pay_ide: i32) -> Vec<PayRecord> {
        let config = parse_ini();
        let mut list: Vec<PayRecord> = Vec::new();
        for item in settle_list.iter() {
            let amount = item.calculate_amount();

            if item.get_num() <= 0 {
                continue;
            }

            list.push(PayRecord {
                pay_sn: None,
                pay_ide: Some(pay_ide),
                stock_sn: item.get_stock_sn(),
                num: item.get_num(),
                pay_time: None,
                amount: amount,
                customer_name: config.name.clone(),
            })
        }
        list
    }
}

pub fn do_settle0(data: String) -> String {
    let settle_list: Vec<Product> = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            return String::from(e.to_string());
        }
    };
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    // 获取购买次数
    let config = parse_ini();
    let pay_ide = conn
        .exec_first(
            "select pay_ide from commissary_transaction_record where customer_name = :name order by pay_ide desc limit 1 ",
            params! {
                "name" => config.name,
            },
        )
        .map(|row| {
            row.map(|pay_ide| {
                match pay_ide {
                    Some(pay_ide) => pay_ide,
                    None => 0,
                }
            })
        })
        .unwrap();

    let pay_ide = match pay_ide {
        Some(pay_ide) => pay_ide,
        None => 0,
    };

    let record_list = PayRecord::from_settle_list(settle_list, pay_ide + 1);
    match conn.exec_batch(
        r"insert into commissary_transaction_record(pay_ide, stock_sn, num, customer_name, pay_time, amount)
            values (:pay_ide, :stock_sn, :num, :customer_name, sysdate(), :amount)",
        record_list.iter().map(|p| params! {
            "pay_ide" => p.pay_ide,
            "stock_sn"=> p.stock_sn,
            "num"=> p.num,
            "customer_name"=> p.customer_name.clone(),
            "amount"=> p.amount,
        })
    ) {
        Ok(_) => JsResult::success(String::from("结算成功")),
        Err(e) => JsResult::<String>::fail(format!("结算失败：{}", e)),
    }
}
