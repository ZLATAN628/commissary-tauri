use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{components::config, JsResult};

use super::{ini_parse::parse_ini, minio, qq_robot, DB_POOL};

#[derive(Deserialize, Serialize, Clone)]
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
    good: Option<i32>,
    bad: Option<i32>,
    state: Option<i32>,
    rate: Option<f32>,
}

impl Product {
    pub fn new(
        stock_sn: Option<i32>,
        product_id: Option<i32>,
        product_name: String,
        product_type: i8,
        count: f32,
        price: f32,
        owner: Option<String>,
        image: Option<String>,
        good: i32,
        bad: i32,
        state: i32,
    ) -> Product {
        let mut rate = 0.0;
        let total = good + bad;
        if total != 0 {
            rate = (good as f32 / total as f32) * 5.0;
        }

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
            good: Some(good),
            bad: Some(bad),
            state: Some(state),
            rate: Some(rate),
        }
    }

    pub fn calculate_amount(&self) -> f32 {
        let num = self.get_num();
        num as f32 * self.price
    }

    pub fn get_num(&self) -> i32 {
        match self.cur {
            Some(num) => num,
            None => 0,
        }
    }

    pub fn get_stock_sn(&self) -> Option<i32> {
        self.stock_sn
    }

    pub fn get_product_name(&self) -> &str {
        self.product_name.as_str()
    }

    fn price(&self) -> f32 {
        self.price
    }

    fn count(&self) -> f32 {
        self.count
    }

    fn get_image(&self) -> String {
        match &self.image {
            Some(e) => e.clone(),
            None => String::new(),
        }
    }

    pub fn get_remain(&self) -> i32 {
        self.count as i32 - self.get_num()
    }
}

pub fn get_product_list0() -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();
    let config = parse_ini();
    let sql = format!("
    select b.stock_sn,
       product_id,
       product_name,
       product_type,
       (b.count - a.num)                                                                                            count,
       price,
       owner,
       image,
       ifnull((select count(*) from commissary_product_comment c where c.stock_sn = b.stock_sn and c.state = 1), 0) good,
       ifnull((select count(*) from commissary_product_comment c where c.stock_sn = b.stock_sn and c.state = 2), 0) bad,
       ifnull((select c.state from commissary_product_comment c where c.stock_sn = b.stock_sn and c.customer_name = '{}'), 0) state
from (select a.stock_sn, sum(ifnull(b.num, 0)) num
      from commissary_product_main a
               left join commissary_transaction_record b on a.stock_sn = b.stock_sn
      group by a.stock_sn) a,
     commissary_product_main b
where a.stock_sn = b.stock_sn
  and (b.count - a.num) > 0
    ", &config.name);
    match conn.query_map(
        sql,
        |(
            stock_sn,
            product_id,
            product_name,
            product_type,
            count,
            price,
            owner,
            image,
            good,
            bad,
            state,
        )| {
            Product::new(
                stock_sn,
                product_id,
                product_name,
                product_type,
                count,
                price,
                owner,
                image,
                good,
                bad,
                state,
            )
        },
    ) {
        Ok(result) => JsResult::success(result),
        Err(e) => JsResult::<String>::fail(format!("查询失败：{}", e)),
    }
}

pub async fn insert_product0(data: String) -> String {
    let obj: Product = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            return String::from(e.to_string());
        }
    };

    // 先上传文件图片
    let image = match minio::upload_file0(obj.get_image(), String::from("product")).await {
        Ok(path) => path,
        Err(err) => return JsResult::<String>::fail(format!("上传文件失败: {}", err)),
    };

    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();

    // 拷贝一份 给消息推送
    let product = obj.clone();

    match conn.exec_drop("insert into commissary_product_main (product_id, product_name, product_type, cost, count, price, stock_time, owner, image)
    values (0, :product_name, :product_type, :cost, :count, :price, sysdate(), :owner, :image)", params! {
        "product_name" => obj.product_name,
        "product_type" => obj.product_type,
        "cost" => obj.cost,
        "count" => obj.count,
        "price" => obj.price,
        "owner" => obj.owner,
        "image" => image,
    }) {
        Ok(_) => send_qq_msg(product).await,
        Err(e) => JsResult::<String>::fail(format!("新增失败：{}", e))
    }
}

pub async fn add_product_count0(num: i32, stock_sn: i32, name: String) -> String {
    let mut conn = DB_POOL
        .get()
        .expect("Error get pool from OneCell<Pool>")
        .get_conn()
        .unwrap();

    conn.exec_drop(
        "update commissary_product_main cm,
        (select * from commissary_product_main where stock_sn = :stock_sn) as b
        set cm.count = b.count + :count
        where cm.stock_sn = b.stock_sn",
        params! {
            "stock_sn" =>stock_sn,
            "count" => num
        },
    )
    .unwrap();

    let param = json!({
        "group_id": config::get_config(1, 771090124),
        "message": format!("商品【{}】库存增加 {} 欢迎大家选购", name, num ),
    });
    match qq_robot::async_post("send_group_msg", &param).await {
        Ok(_) => JsResult::success("新增成功".to_string()),
        Err(err) => {
            return JsResult::<String>::fail(format!("新增成功，但是消息推送失败: {}", err))
        }
    }
}

async fn send_qq_msg(obj: Product) -> String {
    let message = format!(
        "新品上架！【{}】 定价 {} 元，库存 {}，欢迎大家选购",
        obj.get_product_name(),
        obj.price(),
        obj.count()
    );
    let param = json!({
        "group_id": config::get_config(1, 771090124),
        "message": message.as_str(),
    });
    match qq_robot::async_post("send_group_msg", &param).await {
        Ok(_) => (),
        Err(err) => {
            return JsResult::<String>::fail(format!("新增成功，但是消息推送失败: {}", err))
        }
    };
    JsResult::success(String::from("结算成功"))
}
