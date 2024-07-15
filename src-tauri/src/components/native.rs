use mysql::{params, prelude::Queryable};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::JsResult;

use super::{ini_parse::parse_ini, DB_POOL};

pub fn get_native_info0() -> String {
    let mut native = NativeInfo::default();
    if let Ok(Some(addr)) = mac_address::get_mac_address() {
        native.mac_address = addr.to_string().replace(":", "");
        match validate_mac_addr(&native.mac_address) {
            Ok(_) => {}
            Err(e) => return JsResult::<String>::fail(e),
        }
    }
    let value: Value = native.into();
    JsResult::success(value)
}

fn validate_mac_addr(mac_addr: &str) -> Result<(), String> {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();

    let config = parse_ini();

    match conn.query::<String, String>(format!(
        "select name from commissary_user_info where mac_adder = '{}'",
        mac_addr
    )) {
        Ok(data) => {
            if data.is_empty() {
                match conn.exec_drop(
                    format!(
                    "insert into commissary_user_info (mac_adder, name) values (:mac_adder, :name)"
                ),
                    params! {
                        "mac_adder" => mac_addr,
                        "name" => &config.name,
                    },
                ) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("inser error".into()),
                }
            } else if data.len() == 1 {
                if data[0] == config.name {
                    Ok(())
                } else {
                    Err("wrong user".to_string())
                }
            } else {
                Err("query error".to_string())
            }
        }
        Err(e) => {
            println!("error {e:?}");
            Err("query error".to_string())
        }
    }
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct NativeInfo {
    mac_address: String,
}

impl Into<Value> for NativeInfo {
    fn into(self) -> Value {
        json!({
            "macAddress": self.mac_address,
        })
    }
}
