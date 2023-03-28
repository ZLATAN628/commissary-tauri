use mysql::prelude::*;
use serde::{Deserialize, Serialize};

use crate::JsResult;

use super::DB_POOL;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub image: String,
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

    JsResult::success(result)
}
