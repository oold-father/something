use crate::db::{Database, WatchedDirectory, CreateWatchedDirectoryRequest};
use crate::watcher::{DirectoryScanner, ScanConfig, ScanResult};
use crate::error::Result;

/// 获取所有监控目录
#[tauri::command]
pub fn get_watched_directories(
    state: tauri::State<Database>,
) -> std::result::Result<Vec<WatchedDirectory>, String> {
    state.get_watched_directories().map_err(|e| e.to_string())
}

/// 添加监控目录
#[tauri::command]
pub fn add_watched_directory(
    request: CreateWatchedDirectoryRequest,
    state: tauri::State<Database>,
) -> std::result::Result<i64, String> {
    let dir = WatchedDirectory {
        id: None,
        path: request.path,
        recursive: request.recursive,
        filters: request.filters.map(|f| serde_json::to_value(f).unwrap()),
        enabled: true,
        created_at: chrono::Utc::now(),
        last_scanned_at: None,
    };

    state.create_watched_directory(&dir).map_err(|e| e.to_string())
}

/// 移除监控目录
#[tauri::command]
pub fn remove_watched_directory(
    id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    state.delete_watched_directory(id).map_err(|e| e.to_string())
}

/// 更新监控目录
#[tauri::command]
pub fn update_watched_directory(
    id: i64,
    enabled: bool,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    use rusqlite::params;

    let conn = state.conn.lock();
    conn.execute(
        "UPDATE watched_directories SET enabled = ?1 WHERE id = ?2",
        params![enabled as i32, id],
    ).map_err(|e| e.to_string())
}

/// 扫描目录
#[tauri::command]
pub fn scan_directory(
    path: String,
    recursive: bool,
    state: tauri::State<Database>,
) -> std::result::Result<ScanResult, String> {
    let path_obj = std::path::PathBuf::from(&path);

    if !path_obj.exists() {
        return Err("目录不存在".to_string());
    }

    if !path_obj.is_dir() {
        return Err("不是目录".to_string());
    }

    let config = ScanConfig {
        recursive,
        ..Default::default()
    };

    let scanner = DirectoryScanner::new((*state).clone()).with_config(config);
    let result = scanner.scan(&path_obj).map_err(|e| e.to_string())?;

    // 更新目录扫描时间
    if let Some(dir) = state.get_watched_directories()?.into_iter().find(|d| d.path == path) {
        if let Some(id) = dir.id {
            let _ = state.update_directory_scan_time(id);
        }
    }

    Ok(result)
}
