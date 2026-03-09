// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod error;
mod commands;
mod watcher;
mod tagger;

use db::Database;

pub fn run() {
    // 初始化数据库
    let db = Database::new().expect("Failed to initialize database");

    // 注册 Tauri 命令并获取 Builder
    let builder = commands::register_commands(db);

    // 运行 Tauri 应用
    builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
