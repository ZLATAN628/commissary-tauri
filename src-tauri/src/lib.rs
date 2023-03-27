use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
    ParseResult,
};
use ini::Ini;
use mysql::prelude::*;
use mysql::*;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

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
    // 解析配置文件
    parse_ini()
}

pub fn write_user_info0(name: String) -> String {
    let mut config = Ini::new();
    config.with_section(Some("main")).set("name", name);
    config.write_to_file("./config.ini").unwrap();
    String::from("写入成功")
}

pub fn do_settle0(data: String) -> String {
    let obj: Product = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            return String::from(e.to_string());
        }
    };

    println!("{:?}", obj);
    String::from("结算成功")
}

fn parse_ini() -> String {
    let path = Path::new("./config.ini");

    let ini_file = match Ini::load_from_file(path) {
        Ok(ini) => ini,
        Err(e) => {
            File::create(&path).unwrap();
            match Ini::load_from_file(path) {
                Ok(ini) => ini,
                Err(e) => {
                    return format!("加载文件失败：{}", e);
                }
            }
        }
    };
    let result = IniResult::new(ini_file);
    match serde_json::to_string(&result) {
        Ok(s) => s,
        Err(e) => return format!("转换 JSON 失败：{}", e),
    }
}

#[derive(Serialize, Deserialize)]
struct IniResult {
    name: String,
}

impl IniResult {
    pub fn new(ini_file: Ini) -> IniResult {
        let mut res = IniResult {
            name: String::new(),
        };
        for (sec, prop) in ini_file.iter() {
            if let Some(section) = sec {
                if section == "main" {
                    for (k, v) in prop.iter() {
                        if k == "name" {
                            res.name.push_str(v);
                        }
                    }
                }
            }
            println!("Section: {:?}", sec);
        }
        res
    }
}
