// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commissary_tauri::{
    add_comment0, do_settle0, get_carousel_list0, get_pay_record_list0, get_product_list0,
    get_user_info0, insert_product0, upload_file0, write_user_info0, JsResult,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn get_product_list() -> String {
    get_product_list0()
}

#[tauri::command]
fn insert_product(data: String) -> String {
    insert_product0(data)
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
async fn do_settle(data: String) -> String {
    do_settle0(data).await
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
fn add_comment(state: i32, stock_sn: i32) -> String {
    add_comment0(state, stock_sn)
}

#[tauri::command]
async fn upload_file(path: String) -> String {
    match upload_file0(path).await {
        Ok(msg) => msg,
        Err(err) => JsResult::<String>::fail(err.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
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
            add_comment
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
