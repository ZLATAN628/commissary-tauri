mod components;

pub use components::{
    images::get_carousel_list0,
    ini_parse::{get_user_info0, write_user_info0},
    pay::{do_settle0, get_pay_record_list0},
    product::{get_product_list0, insert_product0},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct JsResult<T> {
    code: i32,
    msg: String,
    data: Option<T>,
}

// 统一封装返回给前端的对象
// 规定了泛型类型 T 需要实现 Serialize 这个 trait
impl<T: Serialize> JsResult<T> {
    pub fn success(data: T) -> String {
        let res = JsResult {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        };
        // json 格式化基本不会报错，直接 unwrap 处理 Result 对象。
        // 句尾不加分号代表直接把这个语句的返回值当成函数的返回值
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
