pub mod comments;
pub mod images;
pub mod ini_parse;
pub mod minio;
pub mod pay;
pub mod product;
pub mod qq_robot;

use mysql::*;
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool> = OnceCell::new();
