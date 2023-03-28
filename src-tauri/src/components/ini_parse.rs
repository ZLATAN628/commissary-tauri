use crate::JsResult;

use super::DB_POOL;
use ini::Ini;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct IniResult {
    pub name: String,
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

pub fn parse_ini() -> IniResult {
    let path = Path::new("./config.ini");

    let ini_file = match Ini::load_from_file(path) {
        Ok(ini) => ini,
        Err(_) => {
            match File::create(&path) {
                Ok(_) => (),
                Err(_) => {
                    return IniResult {
                        name: String::from("文件创建失败"),
                    }
                }
            };
            match Ini::load_from_file(path) {
                Ok(ini) => ini,
                Err(_) => {
                    return IniResult {
                        name: String::new(),
                    };
                }
            }
        }
    };
    IniResult::new(ini_file)
}

pub fn write_user_info0(name: String) -> String {
    let mut config = Ini::new();
    config.with_section(Some("main")).set("name", name);
    config.write_to_file("./config.ini").unwrap();
    JsResult::success(String::from("写入成功"))
}

pub fn get_user_info0(flag: i32) -> String {
    // 第一次获取人员信息，初始化数据库连接
    if flag == 0 {
        let config_str;
        {
            let pool =
                match mysql::Pool::new("mysql://root:Ycx19981118.@47.99.168.139:3306/kt_info") {
                    Ok(pool) => pool,
                    Err(e) => {
                        return JsResult::<String>::fail(format!("数据库连接失败：{}", e));
                    }
                };
            let mut conn = pool
                .get_conn()
                .expect(&format!("Error get connection from pool"));
            let mut config: Vec<String> = conn
                .query_map(
                    "select image from commissary_image_save where image_type = 99",
                    |image| image,
                )
                .unwrap();
            config_str = match config.pop() {
                Some(config) => config,
                None => String::from("mysql://root:Ycx19981118.@47.99.168.139:3306/kt_info"),
            };
        }
        match DB_POOL
            .set(mysql::Pool::new(&config_str[..]).expect(&format!("Error connecting to Mysql")))
        {
            Ok(_) => (),
            Err(_) => {
                return JsResult::<String>::fail(format!("数据库连接池对象重复赋值"));
            }
        }
    }

    // 解析配置文件
    let result = parse_ini();
    if result.name == "文件创建失败" {
        JsResult::<String>::fail(format!(
            "配置文件创建失败！！！第一次登录软件，请选择以管理员身份启动！！！"
        ))
    } else {
        JsResult::success(result)
    }
}
