// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commissary_tauri::{
    do_settle0, get_product_list0, get_user_info0, insert_product0, write_user_info0,
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
fn get_user_info() -> String {
    get_user_info0()
}

#[tauri::command]
fn write_user_info(name: String) -> String {
    write_user_info0(name)
}

#[tauri::command]
fn do_settle(data: String) -> String {
    do_settle0(data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            insert_product,
            get_product_list,
            get_user_info,
            do_settle,
            write_user_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}