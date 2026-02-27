use super::models::*;
use super::Database;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::PathBuf;

impl Database {
    /// 创建文件
    pub fn create_file(&self, file: &File) -> Result<i64> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp();

        conn.execute(
            "INSERT INTO files (path, name, extension, size, file_type, created_at, modified_at, accessed_at, status, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                file.path,
                file.name,
                file.extension,
                file.size,
                file.file_type.to_string(),
                file.created_at.timestamp(),
                file.modified_at.timestamp(),
                file.accessed_at.timestamp(),
                file.status.as_str(),
                now,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 根据路径获取文件
    pub fn get_file_by_path(&self, path: &str) -> Result<Option<File>> {
        let conn = self.conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, path, name, extension, size, file_type, created_at, modified_at, accessed_at, status, indexed_at, metadata
             FROM files WHERE path = ?1"
        )?;

        let mut rows = stmt.query(params![path])?;

        if let Some(row) = rows.next()? {
            return Ok(Some(self.row_to_file(row)?));
        }

        Ok(None)
    }

    /// 根据ID获取文件
    pub fn get_file_by_id(&self, id: i64) -> Result<Option<File>> {
        let conn = self.conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, path, name, extension, size, file_type, created_at, modified_at, accessed_at, status, indexed_at, metadata
             FROM files WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            return Ok(Some(self.row_to_file(row)?));
        }

        Ok(None)
    }

    /// 更新文件状态
    pub fn update_file_status(&self, id: i64, status: FileStatus) -> Result<()> {
        let conn = self.conn.lock();

        conn.execute(
            "UPDATE files SET status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;

        Ok(())
    }

    /// 删除文件
    pub fn delete_file(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock();

        conn.execute("DELETE FROM files WHERE id = ?1", params![id])?;

        Ok(())
    }

    /// 获取文件列表
    pub fn get_files(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<File>> {
        let conn = self.conn.lock();

        let sql = if limit.is_some() || offset.is_some() {
            "SELECT id, path, name, extension, size, file_type, created_at, modified_at, accessed_at, status, indexed_at, metadata
             FROM files WHERE status = 'active'
             ORDER BY created_at DESC LIMIT ?1 OFFSET ?2"
        } else {
            "SELECT id, path, name, extension, size, file_type, created_at, modified_at, accessed_at, status, indexed_at, metadata
             FROM files WHERE status = 'active'
             ORDER BY created_at DESC"
        };

        let mut stmt = conn.prepare(sql)?;

        let rows = if limit.is_some() || offset.is_some() {
            stmt.query(params![limit.unwrap_or(100), offset.unwrap_or(0)])?
        } else {
            stmt.query(params![])?
        };

        let mut files = Vec::new();
        for row in rows {
            files.push(self.row_to_file(row)?);
        }

        Ok(files)
    }

    /// 获取文件总数
    pub fn get_file_count(&self) -> Result<i64> {
        let conn = self.conn.lock();

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM files WHERE status = 'active'",
            [],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    /// 创建标签
    pub fn create_tag(&self, tag: &Tag) -> Result<i64> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp();

        conn.execute(
            "INSERT INTO tags (name, display_name, tag_type, color, icon, use_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                tag.name,
                tag.display_name,
                tag.tag_type.as_str(),
                tag.color,
                tag.icon,
                tag.use_count,
                now,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 获取所有标签
    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let conn = self.conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, name, display_name, tag_type, color, icon, use_count, created_at
             FROM tags ORDER BY use_count DESC, created_at DESC"
        )?;

        let mut tags = Vec::new();
        for row in stmt.query([])? {
            tags.push(self.row_to_tag(row)?);
        }

        Ok(tags)
    }

    /// 根据名称获取标签
    pub fn get_tag_by_name(&self, name: &str) -> Result<Option<Tag>> {
        let conn = self.conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, name, display_name, tag_type, color, icon, use_count, created_at
             FROM tags WHERE name = ?1"
        )?;

        let mut rows = stmt.query(params![name])?;

        if let Some(row) = rows.next()? {
            return Ok(Some(self.row_to_tag(row)?));
        }

        Ok(None)
    }

    /// 获取文件的标签
    pub fn get_tags_by_file(&self, file_id: i64) -> Result<Vec<Tag>> {
        let conn = self.conn.lock();

        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.display_name, t.tag_type, t.color, t.icon, t.use_count, t.created_at
             FROM tags t
             JOIN file_tags ft ON t.id = ft.tag_id
             WHERE ft.file_id = ?1
             ORDER BY ft.created_at DESC"
        )?;

        let mut tags = Vec::new();
        for row in stmt.query(params![file_id])? {
            tags.push(self.row_to_tag(row)?);
        }

        Ok(tags)
    }

    /// 添加标签到文件
    pub fn add_tag_to_file(&self, file_id: i64, tag_id: i64, is_auto: bool) -> Result<()> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp();

        conn.execute(
            "INSERT OR REPLACE INTO file_tags (file_id, tag_id, is_auto, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![file_id, tag_id, is_auto as i32, now],
        )?;

        // 更新标签使用计数
        conn.execute(
            "UPDATE tags SET use_count = use_count + 1 WHERE id = ?1",
            params![tag_id],
        )?;

        Ok(())
    }

    /// 根据标签名称添加到文件
    pub fn add_tag_to_file_by_name(&self, file_id: i64, tag_name: &str, is_auto: bool) -> Result<()> {
        // 获取或创建标签
        let tag = match self.get_tag_by_name(tag_name)? {
            Some(t) => t,
            None => {
                // 创建系统标签
                let new_tag = Tag {
                    id: None,
                    name: tag_name.to_string(),
                    display_name: tag_name.to_string(),
                    tag_type: TagType::System,
                    color: "#007ACC".to_string(),
                    icon: None,
                    use_count: 0,
                    created_at: Utc::now(),
                };
                let tag_id = self.create_tag(&new_tag)?;
                Tag {
                    id: Some(tag_id),
                    ..new_tag
                }
            }
        };

        if let Some(tag_id) = tag.id {
            self.add_tag_to_file(file_id, tag_id, is_auto)?;
        }

        Ok(())
    }

    /// 从文件移除标签
    pub fn remove_tag_from_file(&self, file_id: i64, tag_id: i64) -> Result<()> {
        let conn = self.conn.lock();

        // 更新标签使用计数
        conn.execute(
            "UPDATE tags SET use_count = use_count - 1 WHERE id = ?1 AND use_count > 0",
            params![tag_id],
        )?;

        conn.execute(
            "DELETE FROM file_tags WHERE file_id = ?1 AND tag_id = ?2",
            params![file_id, tag_id],
        )?;

        Ok(())
    }

    /// 批量添加标签到文件
    pub fn batch_add_tags(&self, file_ids: &[i64], tag_names: &[String]) -> Result<()> {
        let conn = self.conn.lock();

        let tx = conn.transaction()?;

        for &file_id in file_ids {
            for tag_name in tag_names {
                if let Some(tag) = tx.query_row(
                    "SELECT id FROM tags WHERE name = ?1",
                    params![tag_name],
                    |row| row.get(0),
                ).optional()? {
                    let now = Utc::now().timestamp();
                    tx.execute(
                        "INSERT OR REPLACE INTO file_tags (file_id, tag_id, is_auto, created_at)
                         VALUES (?1, ?2, 0, ?3)",
                        params![file_id, tag, now],
                    )?;

                    tx.execute(
                        "UPDATE tags SET use_count = use_count + 1 WHERE id = ?1",
                        params![tag],
                    )?;
                }
            }
        }

        tx.commit()?;

        Ok(())
    }

    /// 根据标签获取文件
    pub fn get_files_by_tags(&self, tag_names: &[String]) -> Result<Vec<File>> {
        let conn = self.conn.lock();

        let placeholders = tag_names.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!(
            "SELECT DISTINCT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata
             FROM files f
             JOIN file_tags ft ON f.id = ft.file_id
             JOIN tags t ON ft.tag_id = t.id
             WHERE t.name IN ({}) AND f.status = 'active'
             ORDER BY f.created_at DESC",
            placeholders
        );

        let mut stmt = conn.prepare(&sql)?;

        let params: Vec<&rusqlite::types::Value> = tag_names
            .iter()
            .map(|n| rusqlite::types::Value::from(n.as_str()))
            .collect();

        let mut files = Vec::new();
        for row in stmt.query(params.as_slice())? {
            files.push(self.row_to_file(row)?);
        }

        Ok(files)
    }

    /// 创建监控目录
    pub fn create_watched_directory(&self, dir: &WatchedDirectory) -> Result<i64> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp();

        let filters_json = dir.filters.as_ref().map(|f| serde_json::to_string(f).unwrap());

        conn.execute(
            "INSERT INTO watched_directories (path, recursive, filters, enabled, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                dir.path,
                dir.recursive as i32,
                filters_json,
                dir.enabled as i32,
                now,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// 获取所有监控目录
    pub fn get_watched_directories(&self) -> Result<Vec<WatchedDirectory>> {
        let conn = self.conn.lock();

        let mut stmt = conn.prepare(
            "SELECT id, path, recursive, filters, enabled, created_at, last_scanned_at
             FROM watched_directories
             ORDER BY created_at DESC"
        )?;

        let mut dirs = Vec::new();
        for row in stmt.query([])? {
            let filters: Option<String> = row.get(3)?;
            let filters = filters.and_then(|f| serde_json::from_str(&f).ok());

            dirs.push(WatchedDirectory {
                id: Some(row.get(0)?),
                path: row.get(1)?,
                recursive: row.get::<_, i32>(2)? != 0,
                filters,
                enabled: row.get::<_, i32>(4)? != 0,
                created_at: DateTime::from_timestamp(row.get(5)?, 0).unwrap(),
                last_scanned_at: row
                    .get::<_, Option<i64>>(6)?
                    .map(|t| DateTime::from_timestamp(t, 0).unwrap()),
            });
        }

        Ok(dirs)
    }

    /// 删除监控目录
    pub fn delete_watched_directory(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock();

        conn.execute("DELETE FROM watched_directories WHERE id = ?1", params![id])?;

        Ok(())
    }

    /// 更新目录扫描时间
    pub fn update_directory_scan_time(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp();

        conn.execute(
            "UPDATE watched_directories SET last_scanned_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        Ok(())
    }

    /// 获取系统统计
    pub fn get_stats(&self) -> Result<SystemStats> {
        let conn = self.conn.lock();

        let total_files: i64 = conn.query_row(
            "SELECT COUNT(*) FROM files",
            [],
            |row| row.get(0),
        )?;

        let indexed_files: i64 = conn.query_row(
            "SELECT COUNT(*) FROM files WHERE status = 'active'",
            [],
            |row| row.get(0),
        )?;

        let total_tags: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tags",
            [],
            |row| row.get(0),
        )?;

        let watched_directories: i64 = conn.query_row(
            "SELECT COUNT(*) FROM watched_directories WHERE enabled = 1",
            [],
            |row| row.get(0),
        )?;

        Ok(SystemStats {
            total_files,
            indexed_files,
            total_tags,
            watched_directories,
        })
    }

    /// 搜索文件
    pub fn search_files(&self, query: &SearchQuery) -> Result<SearchResultResponse> {
        let conn = self.conn.lock();

        // 构建 FTS 查询
        let fts_query = self.build_fts_query(query)?;

        // 构建基础 SQL
        let mut sql = String::from(
            "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata, fts.rank
             FROM file_tags_fts fts
             JOIN files f ON fts.file_id = f.id
             WHERE file_tags_fts MATCH ?1 AND f.status = 'active'"
        );

        // 添加文件类型过滤
        if let Some(file_type) = &query.file_type_filter {
            sql.push_str(&format!(" AND f.file_type = '{}'", file_type.to_string()));
        }

        sql.push_str(" ORDER BY fts.rank DESC, f.created_at DESC LIMIT ?2 OFFSET ?3");

        let mut stmt = conn.prepare(&sql)?;

        let mut results = Vec::new();
        let mut rows = stmt.query(params![
            fts_query,
            query.limit as i64,
            query.offset as i64
        ])?;

        while let Some(row) = rows.next()? {
            let file = self.row_to_file(row)?;
            let relevance: f32 = row.get(12)?;

            let tags = if let Some(id) = file.id {
                // 重新获取标签（因为行不包含）
                conn.query_row(
                    "SELECT t.id, t.name, t.display_name, t.tag_type, t.color, t.icon, t.use_count, t.created_at
                     FROM tags t
                     JOIN file_tags ft ON t.id = ft.tag_id
                     WHERE ft.file_id = ?1",
                    params![id],
                    |row| self.row_to_tag(row),
                ).optional()?
            } else {
                None
            };

            results.push(SearchResult {
                file,
                tags: if tags.is_some() { vec![tags.unwrap()] } else { vec![] },
                relevance,
            });
        }

        // 获取总数
        let total: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT f.id)
             FROM file_tags_fts fts
             JOIN files f ON fts.file_id = f.id
             WHERE file_tags_fts MATCH ?1 AND f.status = 'active'",
            params![fts_query],
            |row| row.get(0),
        )?;

        Ok(SearchResultResponse {
            results,
            total,
        })
    }

    /// 构建 FTS 查询
    fn build_fts_query(&self, query: &SearchQuery) -> Result<String> {
        match query.operator {
            SearchOperator::And => {
                // AND 逻辑: 所有关键字都必须出现
                let terms: Vec<String> = query.keywords
                    .iter()
                    .map(|k| format!("\"{}\"", k.replace('"', "\"\"")))
                    .collect();
                Ok(terms.join(" "))
            }
            SearchOperator::Or => {
                // OR 逻辑: 任一关键字出现即可
                let terms: Vec<String> = query.keywords
                    .iter()
                    .map(|k| format!("\"{}\"", k.replace('"', "\"\"")))
                    .collect();
                Ok(terms.join(" OR "))
            }
        }
    }

    /// 将数据库行转换为 File
    fn row_to_file(&self, row: &rusqlite::Row) -> Result<File> {
        let file_type_str: String = row.get(4)?;
        let file_type = FileType::from_extension(&file_type_str);

        let status_str: String = row.get(8)?;
        let status = FileStatus::from_str(&status_str);

        let metadata: Option<String> = row.get(11)?;
        let metadata = metadata.and_then(|m| serde_json::from_str(&m).ok());

        Ok(File {
            id: Some(row.get(0)?),
            path: row.get(1)?,
            name: row.get(2)?,
            extension: row.get(3)?,
            size: row.get(5)?,
            file_type,
            created_at: DateTime::from_timestamp(row.get(6)?, 0).unwrap(),
            modified_at: DateTime::from_timestamp(row.get(7)?, 0).unwrap(),
            accessed_at: DateTime::from_timestamp(row.get(7)?, 0).unwrap(),
            status,
            indexed_at: DateTime::from_timestamp(row.get(9)?, 0).unwrap(),
            metadata,
        })
    }

    /// 将数据库行转换为 Tag
    fn row_to_tag(&self, row: &rusqlite::Row) -> Result<Tag> {
        let tag_type_str: String = row.get(3)?;
        let tag_type = TagType::from_str(&tag_type_str);

        Ok(Tag {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            display_name: row.get(2)?,
            tag_type,
            color: row.get(4)?,
            icon: row.get(5)?,
            use_count: row.get(6)?,
            created_at: DateTime::from_timestamp(row.get(7)?, 0).unwrap(),
        })
    }
}
