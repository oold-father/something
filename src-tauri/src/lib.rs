// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod commands;

use db::Database;

pub fn run() {
    // 初始化数据库
    let db = Database::new().expect("Failed to initialize database");

    // 注册 Tauri 命令
    commands::register_commands(db);

    // 运行 Tauri 应用
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
