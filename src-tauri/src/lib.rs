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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PayRecord {
    pay_sn: Option<i32>,
    pay_ide: Option<i32>,
    stock_sn: Option<i32>,
    num: i32,
    customer_name: String,
    pay_time: Option<chrono::NaiveDateTime>,
    amount: f32,
}

impl Product {
    fn calculate_amount(&self) -> f32 {
        let num = self.get_num();
        num as f32 * self.price
    }

    fn get_num(&self) -> i32 {
        match self.cur {
            Some(num) => num,
            None => 0,
        }
    }
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
                stock_sn: item.stock_sn,
                num: item.get_num(),
                pay_time: None,
                amount: amount,
                customer_name: config.name.clone(),
            })
        }
        list
    }
}

pub fn get_product_list0() -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    let sql = "select b.stock_sn,product_id,product_name,product_type,(b.count - a.num) count,price,owner,image
from (select a.stock_sn, sum(ifnull(b.num, 0)) num from commissary_product_main a left join commissary_transaction_record b on a.stock_sn = b.stock_sn group by a.stock_sn) a,
commissary_product_main b where a.stock_sn = b.stock_sn and (b.count - a.num) > 0";
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

pub fn get_user_info0(flag: i32) -> String {
    // 第一次获取人员信息，初始化数据库连接
    if flag == 0 {
        DB_POOL
            .set(
                mysql::Pool::new("mysql://root:Ycx19981118.@47.99.168.139:3306/kt_info")
                    .expect(&format!("Error connecting to Mysql")),
            )
            .unwrap();
    }

    // 解析配置文件
    let result = parse_ini();
    match serde_json::to_string(&result) {
        Ok(s) => s,
        Err(e) => return format!("转换 JSON 失败：{}", e),
    }
}

pub fn write_user_info0(name: String) -> String {
    let mut config = Ini::new();
    config.with_section(Some("main")).set("name", name);
    config.write_to_file("./config.ini").unwrap();
    String::from("写入成功")
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
            row.map(|(pay_ide)| {
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
    conn.exec_batch(
        r"insert into commissary_transaction_record(pay_ide, stock_sn, num, customer_name, pay_time, amount)
            values (:pay_ide, :stock_sn, :num, :customer_name, sysdate(), :amount)",
        record_list.iter().map(|p| params! {
            "pay_ide" => p.pay_ide,
            "stock_sn"=> p.stock_sn,
            "num"=> p.num,
            "customer_name"=> p.customer_name.clone(),
            "amount"=> p.amount,
        })
    ).unwrap();

    String::from("结算成功")
}

fn parse_ini() -> IniResult {
    let path = Path::new("./config.ini");

    let ini_file = match Ini::load_from_file(path) {
        Ok(ini) => ini,
        Err(e) => {
            File::create(&path).unwrap();
            match Ini::load_from_file(path) {
                Ok(ini) => ini,
                Err(e) => {
                    return IniResult {
                        name: String::new(),
                    };
                }
            }
        }
    };
    IniResult::new(ini_file)
}

pub fn get_carousel_list0() -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    let result: Vec<Image> = conn
        .query_map(
            "select image from commissary_image_save where image_type = 1",
            |image| Image { image },
        )
        .unwrap();
    serde_json::to_string(&result).unwrap()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    image: String,
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
        }
        res
    }
}
