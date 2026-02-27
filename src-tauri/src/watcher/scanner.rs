use crate::db::{Database, File, FileStatus, FileType};
use crate::error::Result;
use std::fs;
use std::path::PathBuf;

/// 扫描配置
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// 是否递归扫描子目录
    pub recursive: bool,
    /// 包含的扩展名
    pub extensions: Option<Vec<String>>,
    /// 排除的路径模式
    pub exclude_patterns: Option<Vec<String>>,
    /// 最大扫描深度
    pub max_depth: Option<usize>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        ScanConfig {
            recursive: true,
            extensions: None,
            exclude_patterns: None,
            max_depth: None,
        }
    }
}

/// 目录扫描器
pub struct DirectoryScanner {
    db: Database,
    config: ScanConfig,
}

impl DirectoryScanner {
    /// 创建新的扫描器
    pub fn new(db: Database) -> Self {
        DirectoryScanner {
            db,
            config: ScanConfig::default(),
        }
    }

    /// 设置扫描配置
    pub fn with_config(mut self, config: ScanConfig) -> Self {
        self.config = config;
        self
    }

    /// 扫描指定目录
    pub fn scan(&self, path: &PathBuf) -> Result<ScanResult> {
        let mut result = ScanResult::new(path.clone());

        self.scan_recursive(path, 0, &mut result)?;

        Ok(result)
    }

    /// 递归扫描目录
    fn scan_recursive(&self, path: &PathBuf, depth: usize, result: &mut ScanResult) -> Result<()> {
        // 检查深度限制
        if let Some(max_depth) = self.config.max_depth {
            if depth > max_depth {
                return Ok(());
            }
        }

        // 检查是否应该排除此路径
        if self.should_exclude(path) {
            return Ok(());
        }

        // 读取目录内容
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                result.add_error(path.clone(), e.to_string());
                return Ok(());
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    result.add_error(path.clone(), e.to_string());
                    continue;
                }
            };

            let entry_path = entry.path();

            // 获取文件元数据
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(e) => {
                    result.add_error(entry_path.clone(), e.to_string());
                    continue;
                }
            };

            if metadata.is_file() {
                // 处理文件
                self.process_file(&entry_path, &metadata, result)?;
            } else if metadata.is_dir() && self.config.recursive {
                // 递归处理子目录
                self.scan_recursive(&entry_path, depth + 1, result)?;
            }
        }

        Ok(())
    }

    /// 处理单个文件
    fn process_file(&self, path: &PathBuf, metadata: &fs::Metadata, result: &mut ScanResult) -> Result<()> {
        // 检查扩展名过滤
        if let Some(ref extensions) = self.config.extensions {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if !extensions.iter().any(|e| e.to_lowercase() == ext_str) {
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        }

        // 获取文件信息
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let extension = path.extension()
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

        // 检查文件是否已存在
        let path_str = path.to_string_lossy().to_string();
        let existing_file = self.db.get_file_by_path(&path_str)?;

        let file = File {
            id: existing_file.as_ref().and_then(|f| f.id),
            path: path_str,
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

        if existing_file.is_some() {
            // 文件已存在，检查是否需要更新
            result.updated_files += 1;
        } else {
            // 新文件，添加到数据库
            result.added_files += 1;
            let _ = self.db.create_file(&file);
        }

        result.scanned_files += 1;

        Ok(())
    }

    /// 检查路径是否应该被排除
    fn should_exclude(&self, path: &PathBuf) -> bool {
        if let Some(ref patterns) = self.config.exclude_patterns {
            let path_str = path.to_string_lossy().to_lowercase();
            for pattern in patterns {
                if path_str.contains(&pattern.to_lowercase()) {
                    return true;
                }
            }
        }

        // 排除隐藏目录（以 . 开头）
        if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                if name.starts_with('.') {
                    return true;
                }
            }
        }

        false
    }
}

/// 扫描结果
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// 扫描的目录路径
    pub scan_path: PathBuf,
    /// 扫描的文件总数
    pub scanned_files: usize,
    /// 新增的文件数
    pub added_files: usize,
    /// 更新的文件数
    pub updated_files: usize,
    /// 跳过的文件数
    pub skipped_files: usize,
    /// 错误列表
    pub errors: Vec<(PathBuf, String)>,
}

impl ScanResult {
    pub fn new(path: PathBuf) -> Self {
        ScanResult {
            scan_path: path,
            scanned_files: 0,
            added_files: 0,
            updated_files: 0,
            skipped_files: 0,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, path: PathBuf, error: String) {
        self.errors.push((path, error));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn total_changes(&self) -> usize {
        self.added_files + self.updated_files
    }
}

impl std::fmt::Display for ScanResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScanResult: path={}, scanned={}, added={}, updated={}, errors={}",
            self.scan_path.display(),
            self.scanned_files,
            self.added_files,
            self.updated_files,
            self.errors.len()
        )
    }
}
