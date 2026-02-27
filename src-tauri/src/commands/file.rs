use crate::db::{Database, File, FileType, FileStatus, SystemStats};
use crate::error::Result;
use std::path::Path;

/// 获取文件列表
#[tauri::command]
pub fn get_files(
    limit: Option<i64>,
    offset: Option<i64>,
    state: tauri::State<Database>,
) -> std::result::Result<Vec<File>, String> {
    state.get_files(limit, offset).map_err(|e| e.to_string())
}

/// 根据 ID 获取文件
#[tauri::command]
pub fn get_file_by_id(
    id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<Option<File>, String> {
    state.get_file_by_id(id).map_err(|e| e.to_string())
}

/// 根据路径获取文件
#[tauri::command]
pub fn get_file_by_path(
    path: String,
    state: tauri::State<Database>,
) -> std::result::Result<Option<File>, String> {
    state.get_file_by_path(&path).map_err(|e| e.to_string())
}

/// 添加文件
#[tauri::command]
pub fn add_file(
    path: String,
    state: tauri::State<Database>,
) -> std::result::Result<i64, String> {
    let path_obj = Path::new(&path);

    if !path_obj.exists() {
        return Err("文件不存在".to_string());
    }

    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;

    let name = path_obj.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let extension = path_obj.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();

    let file_type = FileType::from_extension(&extension);

    let created_at = metadata.created()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0).unwrap())
        .unwrap_or_else(chrono::Utc::now);

    let modified_at = metadata.modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0).unwrap())
        .unwrap_or_else(chrono::Utc::now);

    let accessed_at = metadata.accessed()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| chrono::DateTime::from_timestamp(d.as_secs() as i64, 0).unwrap())
        .unwrap_or_else(chrono::Utc::now);

    let file = File {
        id: None,
        path,
        name,
        extension,
        size: metadata.len(),
        file_type,
        created_at,
        modified_at,
        accessed_at,
        status: FileStatus::Active,
        indexed_at: chrono::Utc::now(),
        metadata: None,
    };

    state.create_file(&file).map_err(|e| e.to_string())
}

/// 删除文件
#[tauri::command]
pub fn delete_file(
    id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    state.delete_file(id).map_err(|e| e.to_string())
}

/// 获取系统统计
#[tauri::command]
pub fn get_stats(
    state: tauri::State<Database>,
) -> std::result::Result<SystemStats, String> {
    state.get_stats().map_err(|e| e.to_string())
}
