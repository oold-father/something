mod models;
mod queries;

#[cfg(test)]
mod tests;

pub use models::*;

use crate::error::{AppError, Result};
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::PathBuf;

// 包含数据库表结构的 SQL 文件
const SCHEMA_SQL: &str = include_str!("schema.sql");

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

        // 启用 WAL 模式以提高并发性能（在 execute_batch 之前）
        let _ = conn.execute("PRAGMA journal_mode = WAL;", []);

        // 使用 execute_batch 执行所有 SQL 语句
        conn.execute_batch(SCHEMA_SQL)?;

        // 检查是否需要重建 FTS 表以支持中文分词
        // 检查 FTS 表的分词器配置（通过尝试查询表结构判断）
        // 如果需要更新分词器配置，重建 FTS 表
        let needs_fts_rebuild = conn.query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name='file_tags_content'",
            [],
            |row| {
                let sql: String = row.get(0)?;
                Ok(!sql.contains("tokenize=\"unicode61\""))
            },
        ).unwrap_or(true);

        if needs_fts_rebuild {
            // 删除旧的 FTS 表和相关触发器
            conn.execute_batch(
                r#"
                DROP TRIGGER IF EXISTS fts_file_insert;
                DROP TRIGGER IF EXISTS fts_file_delete;
                DROP TRIGGER IF EXISTS fts_file_update;
                DROP TRIGGER IF EXISTS fts_tag_insert;
                DROP TRIGGER IF EXISTS fts_tag_delete;
                DROP TABLE IF EXISTS file_tags_content;
                "#,
            )?;

            // 使用新的分词器配置重新创建 FTS 表
            conn.execute_batch(
                r#"
                CREATE VIRTUAL TABLE IF NOT EXISTS file_tags_content USING fts5(
                    file_id,
                    file_name,
                    file_path,
                    tag_names,
                    tokenize="unicode61"
                );
                "#,
            )?;

            // 重新创建触发器
            conn.execute_batch(
                r#"
                CREATE TRIGGER IF NOT EXISTS fts_file_insert AFTER INSERT ON files
                BEGIN
                    INSERT INTO file_tags_content(file_id, file_name, file_path, tag_names)
                    VALUES (NEW.id, NEW.name, NEW.path, '');
                END;

                CREATE TRIGGER IF NOT EXISTS fts_file_delete AFTER DELETE ON files
                BEGIN
                    DELETE FROM file_tags_content WHERE file_id = OLD.id;
                END;

                CREATE TRIGGER IF NOT EXISTS fts_file_update AFTER UPDATE ON files
                BEGIN
                    UPDATE file_tags_content SET file_name = NEW.name, file_path = NEW.path
                    WHERE file_id = NEW.id;
                END;

                CREATE TRIGGER IF NOT EXISTS fts_tag_insert AFTER INSERT ON file_tags
                BEGIN
                    UPDATE file_tags_content
                    SET tag_names = (
                        SELECT group_concat(t.name, ',')
                        FROM file_tags ft JOIN tags t ON ft.tag_id = t.id
                        WHERE ft.file_id = NEW.file_id
                    )
                    WHERE file_id = NEW.file_id;
                END;

                CREATE TRIGGER IF NOT EXISTS fts_tag_delete AFTER DELETE ON file_tags
                BEGIN
                    UPDATE file_tags_content
                    SET tag_names = (
                        SELECT group_concat(t.name, ',')
                        FROM file_tags ft JOIN tags t ON ft.tag_id = t.id
                        WHERE ft.file_id = OLD.file_id
                    )
                    WHERE file_id = OLD.file_id;
                END;
                "#,
            )?;

            // 重新填充 FTS 表数据
            conn.execute_batch(
                r#"
                INSERT INTO file_tags_content(file_id, file_name, file_path, tag_names)
                SELECT id, name, path, (
                    SELECT group_concat(t.name, ',')
                    FROM file_tags ft
                    JOIN tags t ON ft.tag_id = t.id
                    WHERE ft.file_id = files.id
                )
                FROM files;
                "#,
            )?;
        }

        // 确保 FTS 表包含现有文件的数据（用于数据迁移）
        // 检查 files 表是否有数据
        let file_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM files",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        // 检查 FTS 表是否有数据
        let fts_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM file_tags_content",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        // 如果 files 有数据而 FTS 表是空的，则填充现有数据
        if file_count > 0 && fts_count == 0 {
            // 插入现有文件到 FTS 表
            conn.execute_batch(
                r#"
                INSERT INTO file_tags_content(file_id, file_name, file_path, tag_names)
                SELECT id, name, path, (
                    SELECT group_concat(t.name, ',')
                    FROM file_tags ft
                    JOIN tags t ON ft.tag_id = t.id
                    WHERE ft.file_id = files.id
                )
                FROM files;
                "#,
            )?;
        }

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    /// 获取数据库文件路径
    fn get_db_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigNotFound("无法获取配置目录".to_string()))?;

        let app_dir = config_dir.join("something");

        Ok(app_dir.join("file_tags.db"))
    }
}
