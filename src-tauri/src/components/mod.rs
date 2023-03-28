pub mod images;
pub mod ini_parse;
pub mod pay;
pub mod product;

use mysql::*;
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool> = OnceCell::new();
