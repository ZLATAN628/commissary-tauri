// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commissary_tauri::{
    add_comment0, add_product_count0, delete_product0, do_settle0, get_arrears_amount0,
    get_carousel_list0, get_native_info0, get_pay_record_list0, get_product_list0,
    get_total_record_list0, get_user_info0, insert_product0, upload_file0, write_user_info0,
    JsResult,
};
use tauri::{utils::config::AppUrl, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_product_list() -> String {
    get_product_list0()
}

#[tauri::command]
async fn insert_product(data: String) -> String {
    insert_product0(data).await
}

#[tauri::command]
fn get_user_info(flag: i32) -> String {
    get_user_info0(flag)
}

#[tauri::command]
fn write_user_info(name: String) -> String {
    write_user_info0(name)
}

#[tauri::command]
async fn do_settle(data: String, payway: i32) -> String {
    do_settle0(data, payway).await
}

#[tauri::command]
fn get_carousel_list() -> String {
    get_carousel_list0()
}

#[tauri::command]
fn get_pay_record_list(name: String) -> String {
    get_pay_record_list0(name)
}

#[tauri::command]
fn get_total_record_list(record_type: i32) -> String {
    get_total_record_list0(record_type)
}

#[tauri::command]
fn add_comment(state: i32, stock_sn: i32) -> String {
    add_comment0(state, stock_sn)
}

#[tauri::command]
fn delete_product(stock_sn: i32) -> String {
    delete_product0(stock_sn)
}

#[tauri::command]
async fn add_product_count(num: i32, stock_sn: i32, name: String) -> String {
    add_product_count0(num, stock_sn, name).await
}

#[tauri::command]
async fn get_arrears_amount() -> String {
    get_arrears_amount0().await
}

#[tauri::command]
async fn upload_file(path: String, file_type: String) -> String {
    match upload_file0(path, file_type).await {
        Ok(msg) => msg,
        Err(err) => JsResult::<String>::fail(err.to_string()),
    }
}

#[tauri::command]
async fn get_native_info() -> String {
    get_native_info0()
}

fn main() {
    let port = 1420;
    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        // 提供给前端调用的rust函数 都需要在这个地方注册
        .invoke_handler(tauri::generate_handler![
            insert_product,
            get_product_list,
            get_user_info,
            do_settle,
            write_user_info,
            get_carousel_list,
            get_pay_record_list,
            upload_file,
            add_comment,
            add_product_count,
            delete_product,
            get_total_record_list,
            get_arrears_amount,
            get_native_info
        ])
        .run(context)
        .expect("error while running tauri application");
}
