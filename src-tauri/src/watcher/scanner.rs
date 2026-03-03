use crate::db::{File, FileStatus, FileType};
use crate::tagger::AutoTagger;
use std::fs;
use std::path::{Path, PathBuf};

/// 扫描配置
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// 是否递归扫描
    pub recursive: bool,
    /// 文件扩展名过滤
    pub extensions: Option<Vec<String>>,
    /// 排除路径模式
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
pub struct DirectoryScanner<'a> {
    db: &'a crate::db::Database,
    tagger: AutoTagger,
    config: ScanConfig,
}

impl<'a> DirectoryScanner<'a> {
    /// 创建新的扫描器
    pub fn new(db: &'a crate::db::Database) -> Self {
        DirectoryScanner {
            db,
            tagger: AutoTagger::new(),
            config: ScanConfig::default(),
        }
    }

    /// 设置扫描配置
    pub fn with_config(mut self, config: ScanConfig) -> Self {
        self.config = config;
        self
    }

    /// 扫描指定目录
    pub fn scan(&self, path: &PathBuf) -> ScanResult {
        let mut result = ScanResult::new(path.clone());

        self.scan_recursive(path, 0, &mut result);

        result
    }

    /// 递归扫描目录
    fn scan_recursive(&self, path: &PathBuf, depth: usize, result: &mut ScanResult) {
        // 检查深度限制
        if let Some(max_depth) = self.config.max_depth {
            if depth > max_depth {
                return;
            }
        }

        // 检查是否应该排除此路径
        if self.should_exclude(path) {
            return;
        }

        // 读取目录内容
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                result.add_error(path.clone(), e.to_string());
                return;
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

            // 处理文件
            if entry_path.is_file() {
                self.process_file(&entry_path, result);
            } else if entry_path.is_dir() && self.config.recursive {
                // 递归处理目录
                self.scan_recursive(&entry_path, depth + 1, result);
            }
        }
    }

    /// 处理单个文件
    fn process_file(&self, path: &PathBuf, result: &mut ScanResult) {
        result.scanned_files += 1;

        // 检查扩展名过滤
        if let Some(ref extensions) = self.config.extensions {
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase());

            if let Some(ref file_ext) = ext {
                if !extensions.contains(&file_ext.to_string()) {
                    result.skipped_files += 1;
                    return;
                }
            }
        }

        // 获取文件元数据
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                result.add_error(path.clone(), e.to_string());
                return;
            }
        };

        let path_str = path.to_string_lossy().to_string();
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        let file_type = FileType::from_extension(&extension);

        // 检查文件是否已存在
        let existing_file = match self.db.get_file_by_path(&path_str) {
            Ok(f) => f,
            Err(_) => {
                result.add_error(path.clone(), "数据库查询失败".to_string());
                return;
            }
        };

        let file = File {
            id: existing_file.as_ref().and_then(|f| f.id),
            path: path_str,
            name,
            extension,
            size: metadata.len() as i64,
            file_type,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            status: FileStatus::Active,
            indexed_at: chrono::Utc::now(),
            metadata: None,
        };

        if existing_file.is_some() {
            result.updated_files += 1;
        } else {
            result.added_files += 1;
        }

        // 创建或更新文件
        let file_id = match self.db.create_file(&file) {
            Ok(id) => id,
            Err(e) => {
                result.add_error(path.clone(), format!("创建文件失败: {}", e));
                return;
            }
        };

        // 为文件生成并添加自动标签
        let tags: Vec<String> = self.tagger.generate_tags(&file);
        for tag_name in tags {
            // 忽略错误，继续处理其他标签
            let _ = self.db.add_tag_to_file_by_name(file_id, &tag_name, true);
        }
    }

    /// 检查是否应该排除此路径
    fn should_exclude(&self, path: &Path) -> bool {
        if let Some(ref patterns) = self.config.exclude_patterns {
            let path_str = path.to_string_lossy().to_string();
            for pattern in patterns {
                if path_str.contains(pattern) {
                    return true;
                }
            }
        }
        false
    }
}

/// 扫描错误
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScanError {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "message")]
    pub message: String,
}

/// 扫描结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    /// 扫描的目录路径
    #[serde(rename = "scanPath")]
    pub scan_path: PathBuf,
    /// 扫描的文件总数
    #[serde(rename = "scannedFiles")]
    pub scanned_files: usize,
    /// 新增的文件数
    #[serde(rename = "addedFiles")]
    pub added_files: usize,
    /// 更新的文件数
    #[serde(rename = "updatedFiles")]
    pub updated_files: usize,
    /// 跳过的文件数
    #[serde(rename = "skippedFiles")]
    pub skipped_files: usize,
    /// 错误列表
    pub errors: Vec<ScanError>,
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
        self.errors.push(ScanError {
            path: path.to_string_lossy().to_string(),
            message: error,
        });
    }

    /// 检查是否有错误（预留功能）
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// 获取变更总数（预留功能）
    #[allow(dead_code)]
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
