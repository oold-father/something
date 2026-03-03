use crate::db::{Database, WatchedDirectory, CreateWatchedDirectoryRequest};
use crate::watcher::{DirectoryScanner, ScanConfig, ScanResult, ScanError};
use rusqlite::Error;
use std::path::PathBuf;

/// 获取所有监控目录
#[tauri::command]
pub fn get_watched_directories(
    state: tauri::State<Database>,
) -> std::result::Result<Vec<WatchedDirectory>, String> {
    state.get_watched_directories().map_err(|e: crate::error::AppError| e.to_string())
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

    state.create_watched_directory(&dir).map_err(|e: crate::error::AppError| e.to_string())
}

/// 移除监控目录
#[tauri::command]
pub fn remove_watched_directory(
    id: i64,
    state: tauri::State<Database>,
) -> std::result::Result<(), String> {
    state.delete_watched_directory(id).map_err(|e: crate::error::AppError| e.to_string())
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
    let _ = conn.execute(
        "UPDATE watched_directories SET enabled = ?1 WHERE id = ?2",
        params![enabled as i32, id],
    ).map_err(|e: Error| e.to_string())?;
    Ok(())
}

/// 扫描目录
#[tauri::command]
pub fn scan_directory(
    path: String,
    recursive: bool,
    state: tauri::State<Database>,
) -> ScanResult {
    let path_obj = std::path::PathBuf::from(&path);

    if !path_obj.exists() {
        return ScanResult {
            scan_path: path_obj,
            scanned_files: 0,
            added_files: 0,
            updated_files: 0,
            skipped_files: 0,
            errors: vec![ScanError {
                path: path.clone(),
                message: "目录不存在".to_string(),
            }],
        };
    }

    if !path_obj.is_dir() {
        return ScanResult {
            scan_path: path_obj,
            scanned_files: 0,
            added_files: 0,
            updated_files: 0,
            skipped_files: 0,
            errors: vec![ScanError {
                path: path.clone(),
                message: "不是目录".to_string(),
            }],
        };
    }

    let config = ScanConfig {
        recursive,
        ..Default::default()
    };

    let scanner = DirectoryScanner::new(&*state).with_config(config);
    let result = scanner.scan(&path_obj);

    // 更新目录扫描时间
    if let Some(dir) = state.get_watched_directories().unwrap_or_default().into_iter().find(|d| d.path == path) {
        if let Some(id) = dir.id {
            let _ = state.update_directory_scan_time(id);
        }
    }

    result
}

/// 批量扫描所有启用的监控目录
#[tauri::command]
pub fn scan_all_directories(
    state: tauri::State<Database>,
) -> std::result::Result<BatchScanResult, String> {
    let directories = state.get_watched_directories()
        .map_err(|e: crate::error::AppError| e.to_string())?;

    let enabled_dirs: Vec<_> = directories
        .into_iter()
        .filter(|d| d.enabled)
        .collect();

    if enabled_dirs.is_empty() {
        return Ok(BatchScanResult {
            total_directories: 0,
            scanned_directories: 0,
            total_files: 0,
            added_files: 0,
            updated_files: 0,
            skipped_files: 0,
            errors: vec![],
        });
    }

    let mut total_result = BatchScanResult::new(enabled_dirs.len());

    for dir in enabled_dirs {
        let path_obj = PathBuf::from(&dir.path);

        if !path_obj.exists() {
            total_result.errors.push(BatchScanError {
                path: dir.path.clone(),
                message: "目录不存在".to_string(),
            });
            continue;
        }

        if !path_obj.is_dir() {
            total_result.errors.push(BatchScanError {
                path: dir.path.clone(),
                message: "不是目录".to_string(),
            });
            continue;
        }

        let config = ScanConfig {
            recursive: dir.recursive,
            ..Default::default()
        };

        let scanner = DirectoryScanner::new(&*state).with_config(config);
        let result = scanner.scan(&path_obj);

        total_result.scanned_directories += 1;
        total_result.total_files += result.scanned_files;
        total_result.added_files += result.added_files;
        total_result.updated_files += result.updated_files;
        total_result.skipped_files += result.skipped_files;

        // 更新目录扫描时间
        if let Some(id) = dir.id {
            let _ = state.update_directory_scan_time(id);
        }

        // 收集错误
        for error in result.errors {
            total_result.errors.push(BatchScanError {
                path: error.path,
                message: error.message,
            });
        }
    }

    Ok(total_result)
}

/// 批量扫描结果
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchScanResult {
    /// 总目录数
    pub total_directories: usize,
    /// 已扫描目录数
    pub scanned_directories: usize,
    /// 总文件数
    pub total_files: usize,
    /// 新增文件数
    pub added_files: usize,
    /// 更新文件数
    pub updated_files: usize,
    /// 跳过文件数
    pub skipped_files: usize,
    /// 错误列表
    pub errors: Vec<BatchScanError>,
}

impl BatchScanResult {
    fn new(total_dirs: usize) -> Self {
        BatchScanResult {
            total_directories: total_dirs,
            scanned_directories: 0,
            total_files: 0,
            added_files: 0,
            updated_files: 0,
            skipped_files: 0,
            errors: vec![],
        }
    }

    /// 检查是否有错误（预留功能）
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 检查是否成功（预留功能）
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        self.scanned_directories > 0 && !self.has_errors()
    }
}

/// 批量扫描错误
#[derive(serde::Serialize)]
pub struct BatchScanError {
    pub path: String,
    pub message: String,
}
