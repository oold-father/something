mod models;
mod queries;

pub use models::*;
pub use queries::*;

use crate::error::{AppError, Result};
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::PathBuf;
use once_cell::sync::Lazy;

// 包含数据库表结构的 SQL 文件
const SCHEMA_SQL: &str = include_str!("schema.sql");

/// 数据库单例
static DB_INSTANCE: Lazy<Mutex<Option<Database>>> = Lazy::new(|| Mutex::new(None));

/// 数据库连接包装器
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// 初始化数据库
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;

        // 确保数据库目录存在
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 打开数据库连接
        let conn = Connection::open(&db_path)?;

        // 执行表结构初始化
        conn.execute_batch(SCHEMA_SQL)?;

        // 启用 WAL 模式以提高并发性能
        conn.execute("PRAGMA journal_mode = WAL;", [])?;

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    /// 获取数据库文件路径
    fn get_db_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigNotFound("无法获取配置目录".to_string()))?;

        let app_dir = config_dir.join("file-tag-manager");

        Ok(app_dir.join("file_tags.db"))
    }

    /// 初始化数据库单例
    pub fn init() -> Result<()> {
        let db = Database::new()?;
        let mut instance = DB_INSTANCE.lock();
        *instance = Some(db);
        Ok(())
    }

    /// 获取数据库单例
    pub fn instance() -> Result<&'static Mutex<Option<Database>>> {
        let instance = DB_INSTANCE.lock();
        if instance.is_none() {
            drop(instance);
            Self::init()?;
        }
        Ok(&DB_INSTANCE)
    }

    /// 初始化默认标签
    pub fn init_default_tags(&self) -> Result<()> {
        let default_tags = vec![
            ("图片", "#F59E0B"),
            ("音频", "#10B981"),
            ("视频", "#EF4444"),
            ("文本", "#6366F1"),
            ("今日文件", "#8B5CF6"),
            ("本周文件", "#EC4899"),
        );

        for (name, color) in default_tags {
            // 检查标签是否已存在
            if self.get_tag_by_name(name)?.is_none() {
                let tag = Tag {
                    id: None,
                    name: name.to_string(),
                    display_name: name.to_string(),
                    tag_type: TagType::System,
                    color: color.to_string(),
                    icon: None,
                    use_count: 0,
                    created_at: chrono::Utc::now(),
                };
                self.create_tag(&tag)?;
            }
        }

        Ok(())
    }
}

// 确保数据库在使用前初始化
pub fn ensure_db_initialized() -> Result<()> {
    Database::init()?;
    let db = DB_INSTANCE.lock();
    if let Some(ref db) = *db {
        db.init_default_tags()?;
    }
    Ok(())
}
