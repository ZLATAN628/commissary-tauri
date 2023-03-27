use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
    ParseResult,
};
use mysql::prelude::*;
use mysql::*;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

static DB_POOL: OnceCell<Pool> = OnceCell::new();
// let static POOL = Pool::new("mysql://root:Ycx19981118.@47.99.168.139:3306/kt_info").unwrap();

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    stock_sn: Option<i32>,
    product_id: Option<i32>,
    product_name: String,
    product_type: i8,
    cost: f32,
    count: f32,
    price: f32,
    stock_time: Option<chrono::NaiveDate>,
    owner: Option<String>,
    image: Option<String>,
    cur: Option<i32>,
}

pub fn get_product_list0() -> String {
    // TODO 待删除
    get_user_info0();
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    let sql = "select stock_sn, product_id, product_name, product_type, count, price, owner, image from commissary_product_main";
    let result = conn
        .query_map(
            sql,
            |(stock_sn, product_id, product_name, product_type, count, price, owner, image)| {
                Product {
                    stock_sn,
                    product_id,
                    product_name,
                    product_type,
                    cost: 0.0,
                    count,
                    price,
                    stock_time: None,
                    owner,
                    image,
                    cur: Some(0),
                }
            },
        )
        .unwrap();
    serde_json::to_string(&result).unwrap()
}

pub fn insert_product0(data: String) -> String {
    let obj: Product = match serde_json::from_str(&data) {
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
    conn.exec_drop("insert into commissary_product_main (product_id, product_name, product_type, cost, count, price, stock_time, owner, image)
    values (0, :product_name, :product_type, :cost, :count, :price, sysdate(), :owner, :image)", params! {
        // "product_id" => obj.product_id,
        "product_name" => obj.product_name,
        "product_type" => obj.product_type,
        "cost" => obj.cost,
        "count" => obj.count,
        "price" => obj.price,
        "owner" => obj.owner,
        "image" => obj.image,
    }).unwrap();
    String::from("Ok")
}

// 只会获取一次人员信息，所以在这初始化数据库连接
pub fn get_user_info0() -> String {
    DB_POOL
        .set(
            mysql::Pool::new("mysql://root:Ycx19981118.@47.99.168.139:3306/kt_info")
                .expect(&format!("Error connecting to Mysql")),
        )
        .unwrap();

    String::from("")
}

pub fn do_settle0(data: String) -> String {
    let obj: Product = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            return String::from(e.to_string());
        }
    };
    String::from("结算成功")
}
