mod components;

pub use components::{
    images::get_carousel_list0,
    ini_parse::{get_user_info0, write_user_info0},
    pay::do_settle0,
    product::{get_product_list0, insert_product0},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct JsResult<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

impl<T: Serialize> JsResult<T> {
    pub fn success(data: T) -> String {
        let res = JsResult {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        };
        serde_json::to_string(&res).unwrap()
    }

    pub fn fail(msg: String) -> String {
        let res: JsResult<String> = JsResult {
            code: 1,
            msg,
            data: None,
        };
        serde_json::to_string(&res).unwrap()
    }
}
