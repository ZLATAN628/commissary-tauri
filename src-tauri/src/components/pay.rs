use super::qq_robot::async_post;
use super::DB_POOL;
use super::{ini_parse::parse_ini, product::*};
use crate::components::config;
use crate::JsResult;
use chrono::{Datelike, Local};
use mysql::{params, prelude::*};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Deserialize, Serialize, Clone)]
pub struct PayRecordShow {
    no: i32,
    pay_time: String,
    info: String,
    amount: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UnPayRecordShow {
    product_name: String,
    num: i32,
    amount: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TotalRecordShow {
    no: i32,
    customer_name: String,
    amount: f32,
}

impl PayRecord {
    fn from_settle_list(settle_list: &Vec<Product>, pay_ide: i32) -> Vec<PayRecord> {
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

/**
 * payway (1=正常支付，2=信用付，3=支付欠款)
 */
pub async fn do_settle0(data: String, payway: i32) -> String {
    let config = parse_ini();
    if payway == 3 {
        return do_credit_pay(&config.name).await;
    }
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
    let pay_ide = conn
        .exec_first(
            "select pay_ide from commissary_transaction_record where customer_name = :name order by pay_ide desc limit 1 ",
            params! {
                "name" => &config.name,
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

    let record_list = PayRecord::from_settle_list(&settle_list, pay_ide + 1);
    match conn.exec_batch(
        r"insert into commissary_transaction_record(pay_ide, stock_sn, num, customer_name, pay_time, amount, pay_state)
            values (:pay_ide, :stock_sn, :num, :customer_name, sysdate(), :amount, :payway)",
        record_list.iter().map(|p| params! {
            "pay_ide" => p.pay_ide,
            "stock_sn"=> p.stock_sn,
            "num"=> p.num,
            "customer_name"=> p.customer_name.clone(),
            "amount"=> p.amount,
            "payway"=>payway
        })
    ) {
        Ok(_) => send_qq_msg(&settle_list).await,
        Err(e) => JsResult::<String>::fail(format!("结算失败：{}", e)),
    }
}

async fn do_credit_pay(username: &str) -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    match conn.exec_drop(
        format!(
            "update commissary_transaction_record set pay_state = 0 where customer_name = :username"
        ),
        params! {
            "username" => username,
        },
    ) {
        Ok(_) => JsResult::success("结算成功"),
        Err(e) => JsResult::<String>::fail(format!("结算失败：{}", e)),
    }
}

pub async fn get_arrears_amount0() -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    let config = parse_ini();
    let sql = format!(
        "select a.product_name, sum(b.num) num, sum(b.amount) amount from commissary_product_main a,
        commissary_transaction_record b where a.stock_sn = b.stock_sn and b.customer_name = '{}'
        and b.pay_state = 1 group by product_name", &config.name);

    match conn.query_map(&sql, |(product_name, num, amount)| UnPayRecordShow {
        product_name,
        num,
        amount,
    }) {
        Ok(v) => JsResult::success(v),
        Err(e) => JsResult::<String>::fail(format!("查询欠款记录失败：{}", e)),
    }
}

// async fn send_qq_msg(name: &str, settle_list: &Vec<Product>) -> String {
//     let mut message = format!(
//         "[TEST] {} 于 {} 购买商品：",
//         name,
//         Local::now().format("%Y年%m月%d日 %H:%M:%S").to_string()
//     );
//     for item in settle_list.iter() {
//         if item.get_num() > 0 {
//             message
//                 .push_str(format!("{} {} 件，", item.get_product_name(), item.get_num()).as_str());
//         }
//     }
//     let compile = Regex::new("，$").unwrap();
//     message = compile.replace(message.as_str(), "。").to_string();
//     let param = json!({
//         "group_id": config::get_config(1, 771090124),
//         "message": message.as_str(),
//     });
//     match async_post("send_group_msg", &param).await {
//         Ok(_) => (),
//         Err(err) => {
//             return JsResult::<String>::fail(format!("结算成功，但是消息推送失败: {}", err))
//         }
//     };
//     JsResult::success(String::from("结算成功"))
// }

async fn send_qq_msg(settle_list: &Vec<Product>) -> String {
    let mut message = "库存警告：".to_string();
    let mut flag = false;
    let warning_count = config::get_config(2, 5);
    for item in settle_list.iter() {
        if item.get_num() > 0 && item.get_remain() < warning_count {
            flag = true;
            message.push_str(
                format!(
                    "{} 仅剩 {} 件，",
                    item.get_product_name(),
                    item.get_remain()
                )
                .as_str(),
            );
        }
    }
    if flag {
        let compile = Regex::new("，$").unwrap();
        message = compile.replace(message.as_str(), "。").to_string();
        message.push_str(" 请尽快补货！");
        let param = json!({
            "group_id": config::get_config(1, 771090124),
            "message": message.as_str(),
        });
        match async_post("send_group_msg", &param).await {
            Ok(_) => (),
            Err(err) => {
                return JsResult::<String>::fail(format!("结算成功，但是消息推送失败: {}", err))
            }
        };
    }
    JsResult::success(String::from("结算成功"))
}

pub fn get_pay_record_list0(name: String) -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();

    let sql = format!("select pay_ide no, date_format(b.pay_time, '%Y-%m-%d %h:%m:%s') pay_time, group_concat('', concat(a.product_name, '*', b.num)) info, sum(b.amount) from commissary_product_main a,
     commissary_transaction_record b where a.stock_sn = b.stock_sn and b.customer_name = '{}' group by pay_ide, b.pay_time order by pay_ide desc", &name);

    match conn.query_map(&sql, |(no, pay_time, info, amount)| PayRecordShow {
        no,
        pay_time,
        info,
        amount,
    }) {
        Ok(v) => JsResult::success(v),
        Err(e) => JsResult::<String>::fail(format!("查询支付记录失败：{}", e)),
    }
}

pub fn get_total_record_list0(record_type: i32) -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();

    let mut record_str = String::new();
    if record_type == 1 {
        let dt = Local::now();
        record_str = format!("where pay_time > '{}-{}-01' ", dt.year(), dt.month());
    }
    let sql = format!(
        "
        SELECT @rownum := @rownum + 1 as no,
        a.*
        FROM (select sum(amount) amount, customer_name
        from commissary_transaction_record
        {}
        group by customer_name
        order by amount desc) a,
        (SELECT @rownum := 0) b
        ORDER BY no",
        record_str
    );

    match conn.query_map(&sql, |(no, amount, customer_name)| TotalRecordShow {
        no,
        customer_name,
        amount,
    }) {
        Ok(v) => JsResult::success(v),
        Err(e) => JsResult::<String>::fail(format!("查询支付榜单失败：{}", e)),
    }
}
