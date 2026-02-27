use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// 文件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    /// 图片文件
    Image,
    /// 音频文件
    Audio,
    /// 视频文件
    Video,
    /// 文本文件
    Text,
    /// 二进制文件
    Binary,
    /// 其他文件
    Other,
}

impl FileType {
    /// 从文件扩展名推断文件类型
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            // 图片
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" | "ico" | "tiff" => FileType::Image,
            // 音频
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma" | "opus" => FileType::Audio,
            // 视频
            "mp4" | "avi" | "mkv" | "mov" | "webm" | "flv" | "wmv" | "m4v" => FileType::Video,
            // 文本
            "txt" | "md" | "json" | "xml" | "yaml" | "yml" | "csv" | "log" | "toml" | "ini"
            | "cfg" | "conf" | "rtf" => FileType::Text,
            // 二进制/压缩包
            "exe" | "dll" | "so" | "bin" | "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => FileType::Binary,
            _ => FileType::Other,
        }
    }

    /// 获取文件类型的中文名称
    pub fn display_name(&self) -> &'static str {
        match self {
            FileType::Image => "图片",
            FileType::Audio => "音频",
            FileType::Video => "视频",
            FileType::Text => "文本",
            FileType::Binary => "二进制",
            FileType::Other => "其他",
        }
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

/// 文件状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    /// 活跃状态
    Active,
    /// 已删除
    Deleted,
    /// 已移动
    Moved,
}

impl FileStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileStatus::Active => "active",
            FileStatus::Deleted => "deleted",
            FileStatus::Moved => "moved",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "active" => FileStatus::Active,
            "deleted" => FileStatus::Deleted,
            "moved" => FileStatus::Moved,
            _ => FileStatus::Active,
        }
    }
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: Option<i64>,
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: i64,
    pub file_type: FileType,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub status: FileStatus,
    pub indexed_at: DateTime<Utc>,
    pub metadata: Option<JsonValue>,
}

/// 标签类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TagType {
    /// 系统自动生成的标签
    System,
    /// 用户自定义标签
    Custom,
}

impl TagType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagType::System => "system",
            TagType::Custom => "custom",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "system" => TagType::System,
            "custom" => TagType::Custom,
            _ => TagType::Custom,
        }
    }
}

/// 标签信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub display_name: String,
    pub tag_type: TagType,
    pub color: String,
    pub icon: Option<String>,
    pub use_count: i64,
    pub created_at: DateTime<Utc>,
}

/// 文件与标签的关联
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTag {
    pub file_id: i64,
    pub tag_id: i64,
    pub is_auto: bool,
    pub created_at: DateTime<Utc>,
}

/// 监控目录信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedDirectory {
    pub id: Option<i64>,
    pub path: String,
    pub recursive: bool,
    pub filters: Option<JsonValue>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_scanned_at: Option<DateTime<Utc>>,
}

/// 目录过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryFilters {
    pub extensions: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: File,
    pub tags: Vec<Tag>,
    pub relevance: f32,
}

/// 搜索查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keywords: Vec<String>,
    pub operator: SearchOperator,
    pub file_type_filter: Option<FileType>,
    pub limit: usize,
    pub offset: usize,
}

/// 搜索运算符
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SearchOperator {
    /// AND 运算：所有关键字都必须匹配
    And,
    /// OR 运算：任一关键字匹配即可
    Or,
}

impl SearchOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchOperator::And => "AND",
            SearchOperator::Or => "OR",
        }
    }
}

/// 搜索结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultResponse {
    pub results: Vec<SearchResult>,
    pub total: i64,
}

/// 系统统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub total_files: i64,
    pub indexed_files: i64,
    pub total_tags: i64,
    pub watched_directories: i64,
}

/// 新建标签请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub display_name: String,
    pub color: String,
    pub icon: Option<String>,
}

/// 新建监控目录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWatchedDirectoryRequest {
    pub path: String,
    pub recursive: bool,
    pub filters: Option<DirectoryFilters>,
}
