use super::models::*;
use super::Database;
use crate::error::{AppError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, OptionalExtension};

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

        // 使用 JOIN 查询一次性获取文件及其标签
        let mut stmt = conn.prepare(
            "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata,
                    t.id as tag_id, t.name as tag_name, t.display_name as tag_display_name, t.tag_type as tag_type, t.color as tag_color, t.icon as tag_icon, t.use_count as tag_use_count, t.created_at as tag_created_at
             FROM files f
             LEFT JOIN file_tags ft ON f.id = ft.file_id
             LEFT JOIN tags t ON ft.tag_id = t.id
             WHERE f.path = ?1"
        )?;

        let mut rows = stmt.query(params![path])?;

        let mut file_map: std::collections::HashMap<i64, (File, Vec<Tag>)> = std::collections::HashMap::new();
        self.collect_files_with_tags(&mut rows, &mut file_map)?;

        if file_map.is_empty() {
            Ok(None)
        } else {
            let (_file, tags) = file_map.into_values().next().unwrap();
            Ok(Some(File { tags: Some(tags), .._file }))
        }
    }

    /// 根据ID获取文件
    pub fn get_file_by_id(&self, id: i64) -> Result<Option<File>> {
        let conn = self.conn.lock();

        // 使用 JOIN 查询一次性获取文件及其标签
        let mut stmt = conn.prepare(
            "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata,
                    t.id as tag_id, t.name as tag_name, t.display_name as tag_display_name, t.tag_type as tag_type, t.color as tag_color, t.icon as tag_icon, t.use_count as tag_use_count, t.created_at as tag_created_at
             FROM files f
             LEFT JOIN file_tags ft ON f.id = ft.file_id
             LEFT JOIN tags t ON ft.tag_id = t.id
             WHERE f.id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        let mut file_map: std::collections::HashMap<i64, (File, Vec<Tag>)> = std::collections::HashMap::new();
        self.collect_files_with_tags(&mut rows, &mut file_map)?;

        if file_map.is_empty() {
            Ok(None)
        } else {
            let (_file, tags) = file_map.into_values().next().unwrap();
            Ok(Some(File { tags: Some(tags), .._file }))
        }
    }

    /// 更新文件状态（预留功能）
    #[allow(dead_code)]
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

        // 使用 JOIN 查询一次性获取文件及其标签
        let sql = if limit.is_some() || offset.is_some() {
            "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata,
                    t.id as tag_id, t.name as tag_name, t.display_name as tag_display_name, t.tag_type as tag_type, t.color as tag_color, t.icon as tag_icon, t.use_count as tag_use_count, t.created_at as tag_created_at
             FROM files f
             LEFT JOIN file_tags ft ON f.id = ft.file_id
             LEFT JOIN tags t ON ft.tag_id = t.id
             WHERE f.status = 'active'
             ORDER BY f.created_at DESC LIMIT ?1 OFFSET ?2"
        } else {
            "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata,
                    t.id as tag_id, t.name as tag_name, t.display_name as tag_display_name, t.tag_type as tag_type, t.color as tag_color, t.icon as tag_icon, t.use_count as tag_use_count, t.created_at as tag_created_at
             FROM files f
             LEFT JOIN file_tags ft ON f.id = ft.file_id
             LEFT JOIN tags t ON ft.tag_id = t.id
             WHERE f.status = 'active'
             ORDER BY f.created_at DESC"
        };

        let mut stmt = conn.prepare(sql)?;

        let mut file_map: std::collections::HashMap<i64, (File, Vec<Tag>)> = std::collections::HashMap::new();

        if limit.is_some() || offset.is_some() {
            let mut rows = stmt.query(params![limit.unwrap_or(100), offset.unwrap_or(0)])?;
            self.collect_files_with_tags(&mut rows, &mut file_map)?;
        } else {
            let mut rows = stmt.query(params![])?;
            self.collect_files_with_tags(&mut rows, &mut file_map)?;
        }

        // 转换为 Vec 并按 created_at 排序
        let mut files: Vec<File> = file_map.into_values()
            .map(|(file, tags)| File { tags: Some(tags), ..file })
            .collect();
        files.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(files)
    }

    /// 获取文件总数（预留功能）
    #[allow(dead_code)]
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

        let icon_value: Option<&str> = tag.icon.as_ref().map(|s| s.as_str());

        conn.execute(
            "INSERT INTO tags (name, display_name, tag_type, color, icon, use_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                tag.name,
                tag.display_name,
                tag.tag_type.as_str(),
                tag.color,
                if let Some(icon) = icon_value { icon } else { "" },
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
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
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
        let mut rows = stmt.query(params![file_id])?;
        while let Some(row) = rows.next()? {
            tags.push(self.row_to_tag(row)?);
        }

        Ok(tags)
    }

    /// 添加标签到文件
    pub fn add_tag_to_file(&self, file_id: i64, tag_id: i64, is_auto: bool) -> Result<()> {
        let conn = self.conn.lock();
        let now = Utc::now().timestamp();

        println!("[DEBUG] add_tag_to_file: file_id={}, tag_id={}, is_auto={}", file_id, tag_id, is_auto);

        // 使用 INSERT OR IGNORE 避免重复添加时重复递增计数
        let rows_affected = conn.execute(
            "INSERT OR IGNORE INTO file_tags (file_id, tag_id, is_auto, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![file_id, tag_id, is_auto as i32, now],
        )?;

        println!("[DEBUG] INSERT OR IGNORE rows_affected: {}", rows_affected);

        // 仅当成功插入新记录时才更新标签使用计数
        if rows_affected > 0 {
            conn.execute(
                "UPDATE tags SET use_count = use_count + 1 WHERE id = ?1",
                params![tag_id],
            )?;
            println!("[DEBUG] Updated use_count for tag_id: {}", tag_id);
        } else {
            println!("[DEBUG] Skipping use_count update (duplicate)");
        }

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
        let mut conn = self.conn.lock();

        let tx = conn.transaction()?;

        for &file_id in file_ids {
            for tag_name in tag_names {
                if let Some(tag) = tx.query_row(
                    "SELECT id FROM tags WHERE name = ?1",
                    params![tag_name],
                    |row| row.get::<_, i64>(0),
                ).optional()? {
                    let now = Utc::now().timestamp();
                    // 使用 INSERT OR IGNORE 避免重复添加时重复递增计数
                    let rows_affected = tx.execute(
                        "INSERT OR IGNORE INTO file_tags (file_id, tag_id, is_auto, created_at)
                         VALUES (?1, ?2, 0, ?3)",
                        params![file_id, tag, now],
                    )?;

                    println!("[DEBUG] batch_add_tags: file_id={}, tag={}, rows_affected={}", file_id, tag_name, rows_affected);

                    // 仅当成功插入新记录时才更新标签使用计数
                    if rows_affected > 0 {
                        tx.execute(
                            "UPDATE tags SET use_count = use_count + 1 WHERE id = ?1",
                            params![tag],
                        )?;
                        println!("[DEBUG] Updated use_count for tag_id: {}", tag);
                    } else {
                        println!("[DEBUG] Skipping use_count update (duplicate)");
                    }
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
            "SELECT DISTINCT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata,
                    t.id as tag_id, t.name as tag_name, t.display_name as tag_display_name, t.tag_type as tag_type, t.color as tag_color, t.icon as tag_icon, t.use_count as tag_use_count, t.created_at as tag_created_at
             FROM files f
             JOIN file_tags ft ON f.id = ft.file_id
             JOIN tags t ON ft.tag_id = t.id
             LEFT JOIN file_tags ft2 ON f.id = ft2.file_id
             LEFT JOIN tags t2 ON ft2.tag_id = t2.id
             WHERE t.name IN ({}) AND f.status = 'active'
             ORDER BY f.created_at DESC",
            placeholders
        );

        let mut stmt = conn.prepare(&sql)?;

        // 使用 rusqlite 的 params_from_iter
        let params: Vec<&dyn rusqlite::ToSql> = tag_names.iter().map(|n| n as &dyn rusqlite::ToSql).collect();

        let mut file_map: std::collections::HashMap<i64, (File, Vec<Tag>)> = std::collections::HashMap::new();
        let mut rows = stmt.query(params.as_slice())?;
        self.collect_files_with_tags(&mut rows, &mut file_map)?;

        // 转换为 Vec 并按 created_at 排序
        let mut files: Vec<File> = file_map.into_values()
            .map(|(file, tags)| File { tags: Some(tags), ..file })
            .collect();
        files.sort_by(|a, b| b.created_at.cmp(&a.created_at));

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
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let filters_raw: Option<String> = row.get(3)?;
            let filters: Option<serde_json::Value> = filters_raw.and_then(|f| serde_json::from_str(&f).ok());

            let created_at_ts: i64 = row.get(5)?;
            let created_at = DateTime::from_timestamp(created_at_ts, 0)
                .ok_or_else(|| AppError::Unknown(format!("无效的 created_at 时间戳: {}", created_at_ts)))?;

            let last_scanned_at: Option<DateTime<Utc>> = row
                .get::<_, Option<i64>>(6)?
                .and_then(|t| DateTime::from_timestamp(t, 0));

            dirs.push(WatchedDirectory {
                id: Some(row.get(0)?),
                path: row.get(1)?,
                recursive: row.get::<_, i32>(2)? != 0,
                filters,
                enabled: row.get::<_, i32>(4)? != 0,
                created_at,
                last_scanned_at,
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

    /// 搜索文件 - 支持混合搜索策略（中文用LIKE，英文数字用FTS）
    pub fn search_files(&self, query: &SearchQuery) -> Result<SearchResultResponse> {
        let conn = self.conn.lock();

        // 检测关键字是否包含中文
        let has_chinese = query.keywords.iter().any(|k| self.contains_chinese(k));

        // 构建标签过滤子句
        let tag_filter = if let Some(ref tags) = query.tags {
            if !tags.is_empty() {
                let placeholders = tags.iter().map(|_| "?").collect::<Vec<_>>().join(",");
                Some(format!("EXISTS (SELECT 1 FROM file_tags ft JOIN tags t ON ft.tag_id = t.id WHERE ft.file_id = f.id AND t.name IN ({}))", placeholders))
            } else {
                None
            }
        } else {
            None
        };

        let mut results = Vec::new();
        let total: i64;

        if has_chinese {
            // 中文搜索：使用 LIKE 进行模糊匹配
            let like_conditions: Vec<String> = query.keywords
                .iter()
                .map(|k| format!("(f.name LIKE '%{}%' OR f.path LIKE '%{}%' OR f.path LIKE '%{}%')",
                    self.escape_like(k), self.escape_like(k), self.escape_like(k)))
                .collect();

            let like_clause = match query.operator {
                SearchOperator::And => like_conditions.join(" AND "),
                SearchOperator::Or => like_conditions.join(" OR "),
            };

            let mut sql = format!(
                "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata, 0.0 as relevance
                 FROM files f
                 WHERE ({}) AND f.status = 'active'",
                like_clause
            );

            // 添加文件类型过滤
            if let Some(file_type) = &query.file_type_filter {
                sql.push_str(&format!(" AND f.file_type = '{}'", file_type.to_string()));
            }

            // 添加标签过滤
            if let Some(ref tag_filter) = tag_filter {
                sql.push_str(&format!(" AND {}", tag_filter));
            }

            sql.push_str(&format!(" ORDER BY f.created_at DESC LIMIT {} OFFSET {}",
                query.limit, query.offset));

            let mut stmt = conn.prepare(&sql)?;

            // 准备查询参数
            let mut query_params: Vec<&dyn rusqlite::ToSql> = Vec::new();
            if let Some(ref tags) = query.tags {
                for tag in tags {
                    query_params.push(tag);
                }
            }
            let mut rows = stmt.query(query_params.as_slice())?;

            while let Some(row) = rows.next()? {
                let file = self.row_to_file(row)?;
                let relevance: f32 = row.get(12)?;

                let tags = self.get_tags_for_file(&conn, file.id)?;

                results.push(SearchResult {
                    file,
                    tags,
                    relevance,
                });
            }

            // 获取总数
            let count_sql = format!(
                "SELECT COUNT(DISTINCT f.id)
                 FROM files f
                 WHERE ({}) AND f.status = 'active'",
                like_clause
            );

            // 在总数查询中也添加标签过滤
            let mut count_sql_with_filters = count_sql;
            if let Some(file_type) = &query.file_type_filter {
                count_sql_with_filters.push_str(&format!(" AND f.file_type = '{}'", file_type.to_string()));
            }
            if let Some(ref tag_filter) = tag_filter {
                count_sql_with_filters.push_str(&format!(" AND {}", tag_filter));
            }

            total = conn.query_row(&count_sql_with_filters, query_params.as_slice(), |row| row.get(0))?;

        } else {
            // 英文/数字搜索：使用 FTS 全文搜索
            let fts_query = self.build_fts_query(query)?;

            let mut sql = String::from(
                "SELECT f.id, f.path, f.name, f.extension, f.size, f.file_type, f.created_at, f.modified_at, f.accessed_at, f.status, f.indexed_at, f.metadata, bm25(file_tags_content)
                 FROM files f
                 JOIN file_tags_content ON f.id = file_tags_content.file_id
                 WHERE file_tags_content MATCH ?1 AND f.status = 'active'"
            );

            // 添加文件类型过滤
            if let Some(file_type) = &query.file_type_filter {
                sql.push_str(&format!(" AND f.file_type = '{}'", file_type.to_string()));
            }

            // 添加标签过滤
            if let Some(ref tag_filter) = tag_filter {
                sql.push_str(&format!(" AND {}", tag_filter));
            }

            sql.push_str(" ORDER BY bm25(file_tags_content) DESC, f.created_at DESC LIMIT ?2 OFFSET ?3");

            let mut stmt = conn.prepare(&sql)?;

            // 准备查询参数
            let limit_i64 = query.limit as i64;
            let offset_i64 = query.offset as i64;
            let mut query_params: Vec<&dyn rusqlite::ToSql> = vec![&fts_query];
            if let Some(ref tags) = query.tags {
                for tag in tags {
                    query_params.push(tag);
                }
            }
            query_params.push(&limit_i64);
            query_params.push(&offset_i64);

            let mut rows = stmt.query(query_params.as_slice())?;

            while let Some(row) = rows.next()? {
                let file = self.row_to_file(row)?;
                let relevance: f32 = row.get(12)?;

                let tags = self.get_tags_for_file(&conn, file.id)?;

                results.push(SearchResult {
                    file,
                    tags,
                    relevance,
                });
            }

            // 获取总数
            let mut count_sql = String::from(
                "SELECT COUNT(DISTINCT f.id)
                 FROM files f
                 JOIN file_tags_content ON f.id = file_tags_content.file_id
                 WHERE file_tags_content MATCH ?1 AND f.status = 'active'"
            );

            // 在总数查询中也添加文件类型和标签过滤
            if let Some(file_type) = &query.file_type_filter {
                count_sql.push_str(&format!(" AND f.file_type = '{}'", file_type.to_string()));
            }
            if let Some(ref tag_filter) = tag_filter {
                count_sql.push_str(&format!(" AND {}", tag_filter));
            }

            // 准备总数查询参数
            let mut count_params: Vec<&dyn rusqlite::ToSql> = vec![&fts_query];
            if let Some(ref tags) = query.tags {
                for tag in tags {
                    count_params.push(tag);
                }
            }

            total = conn.query_row(&count_sql, count_params.as_slice(), |row| row.get(0))?;
        }

        Ok(SearchResultResponse {
            results,
            total,
        })
    }

    /// 检测字符串是否包含中文字符
    fn contains_chinese(&self, text: &str) -> bool {
        text.chars().any(|c| {
            let code = c as u32;
            // 基本汉字范围：\u4e00-\u9fff
            (0x4e00..=0x9fff).contains(&code) ||
            // 扩展汉字范围A：\u3400-\u4dbf
            (0x3400..=0x4dbf).contains(&code) ||
            // 扩展汉字范围B：\u20000-\u2a6df
            (0x20000..=0x2a6df).contains(&code) ||
            // CJK 标点符号
            (0x3000..=0x303f).contains(&code) ||
            (0xff00..=0xffef).contains(&code)
        })
    }

    /// 转义 LIKE 语句中的特殊字符
    fn escape_like(&self, text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    }

    /// 获取文件的标签列表
    fn get_tags_for_file(&self, conn: &rusqlite::Connection, file_id: Option<i64>) -> Result<Vec<Tag>> {
        if let Some(id) = file_id {
            let mut tags = Vec::new();
            let mut stmt = conn.prepare(
                "SELECT t.id, t.name, t.display_name, t.tag_type, t.color, t.icon, t.use_count, t.created_at
                 FROM tags t
                 JOIN file_tags ft ON t.id = ft.tag_id
                 WHERE ft.file_id = ?1"
            )?;
            let mut rows = stmt.query(params![id])?;

            while let Some(row) = rows.next()? {
                let tag_type_str: String = row.get(3)?;
                let tag_type = TagType::from_str(&tag_type_str);
                let created_at_ts: i64 = row.get(7)?;
                let created_at = DateTime::from_timestamp(created_at_ts, 0)
                    .ok_or_else(|| AppError::Unknown(format!("无效的 created_at 时间戳: {}", created_at_ts)))?;
                tags.push(Tag {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    display_name: row.get(2)?,
                    tag_type,
                    color: row.get(4)?,
                    icon: row.get(5)?,
                    use_count: row.get(6)?,
                    created_at,
                });
            }
            Ok(tags)
        } else {
            Ok(vec![])
        }
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
        // SQL列顺序: id(0), path(1), name(2), extension(3), size(4), file_type(5),
        //            created_at(6), modified_at(7), accessed_at(8), status(9), indexed_at(10), metadata(11)
        let file_type_str: String = row.get(5)?;
        let file_type = FileType::from_extension(&file_type_str);

        let status_str: String = row.get(9)?;
        let status = FileStatus::from_str(&status_str);

        let created_at_ts: i64 = row.get(6)?;
        let modified_at_ts: i64 = row.get(7)?;
        let accessed_at_ts: i64 = row.get(8)?;
        let indexed_at_ts: i64 = row.get(10)?;

        let created_at = DateTime::from_timestamp(created_at_ts, 0)
            .ok_or_else(|| AppError::Unknown(format!("无效的 created_at 时间戳: {}", created_at_ts)))?;
        let modified_at = DateTime::from_timestamp(modified_at_ts, 0)
            .ok_or_else(|| AppError::Unknown(format!("无效的 modified_at 时间戳: {}", modified_at_ts)))?;
        let accessed_at = DateTime::from_timestamp(accessed_at_ts, 0)
            .ok_or_else(|| AppError::Unknown(format!("无效的 accessed_at 时间戳: {}", accessed_at_ts)))?;
        let indexed_at = DateTime::from_timestamp(indexed_at_ts, 0)
            .ok_or_else(|| AppError::Unknown(format!("无效的 indexed_at 时间戳: {}", indexed_at_ts)))?;

        let metadata: Option<String> = row.get(11)?;
        let metadata = metadata.and_then(|m| serde_json::from_str(&m).ok());

        Ok(File {
            id: Some(row.get(0)?),
            path: row.get(1)?,
            name: row.get(2)?,
            extension: row.get(3)?,
            size: row.get(4)?,
            file_type,
            created_at,
            modified_at,
            accessed_at,
            status,
            indexed_at,
            metadata,
            tags: None,
        })
    }

    /// 将数据库行转换为 Tag
    fn row_to_tag(&self, row: &rusqlite::Row) -> Result<Tag> {
        let tag_type_str: String = row.get(3)?;
        let tag_type = TagType::from_str(&tag_type_str);
        let created_at_ts: i64 = row.get(7)?;
        let created_at = DateTime::from_timestamp(created_at_ts, 0)
            .ok_or_else(|| AppError::Unknown(format!("无效的时间戳: {}", created_at_ts)))?;

        Ok(Tag {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            display_name: row.get(2)?,
            tag_type,
            color: row.get(4)?,
            icon: row.get(5)?,
            use_count: row.get(6)?,
            created_at,
        })
    }

    /// 从 JOIN 查询结果中收集文件及其标签（避免重复获取锁导致死锁）
    fn collect_files_with_tags(
        &self,
        rows: &mut rusqlite::Rows<'_>,
        file_map: &mut std::collections::HashMap<i64, (File, Vec<Tag>)>,
    ) -> Result<()> {
        while let Some(row) = rows.next()? {
            // 检查 file_id 是否存在（LEFT JOIN 可能为 NULL）
            let file_id_opt: Option<i64> = row.get(0)?;
            if file_id_opt.is_none() {
                continue;
            }
            let file_id = file_id_opt.unwrap();

            // 检查 tag_id 是否存在（文件可能没有标签）
            let tag_id_opt: Option<i64> = row.get(12)?;

            // 如果文件不在 map 中，先添加文件
            if !file_map.contains_key(&file_id) {
                // 文件字段索引: id(0), path(1), name(2), extension(3), size(4), file_type(5),
                //              created_at(6), modified_at(7), accessed_at(8), status(9), indexed_at(10), metadata(11)
                let file_type_str: String = row.get(5)?;
                let file_type = FileType::from_extension(&file_type_str);

                let status_str: String = row.get(9)?;
                let status = FileStatus::from_str(&status_str);

                let created_at_ts: i64 = row.get(6)?;
                let modified_at_ts: i64 = row.get(7)?;
                let accessed_at_ts: i64 = row.get(8)?;
                let indexed_at_ts: i64 = row.get(10)?;

                let created_at = DateTime::from_timestamp(created_at_ts, 0)
                    .ok_or_else(|| AppError::Unknown(format!("无效的 created_at 时间戳: {}", created_at_ts)))?;
                let modified_at = DateTime::from_timestamp(modified_at_ts, 0)
                    .ok_or_else(|| AppError::Unknown(format!("无效的 modified_at 时间戳: {}", modified_at_ts)))?;
                let accessed_at = DateTime::from_timestamp(accessed_at_ts, 0)
                    .ok_or_else(|| AppError::Unknown(format!("无效的 accessed_at 时间戳: {}", accessed_at_ts)))?;
                let indexed_at = DateTime::from_timestamp(indexed_at_ts, 0)
                    .ok_or_else(|| AppError::Unknown(format!("无效的 indexed_at 时间戳: {}", indexed_at_ts)))?;

                let metadata: Option<String> = row.get(11)?;
                let metadata = metadata.and_then(|m| serde_json::from_str(&m).ok());

                let file = File {
                    id: Some(file_id),
                    path: row.get(1)?,
                    name: row.get(2)?,
                    extension: row.get(3)?,
                    size: row.get(4)?,
                    file_type,
                    created_at,
                    modified_at,
                    accessed_at,
                    status,
                    indexed_at,
                    metadata,
                    tags: None,
                };
                file_map.insert(file_id, (file, Vec::new()));
            }

            // 如果有标签，添加到标签列表
            if let Some(_tag_id) = tag_id_opt {
                // 标签字段索引: tag_id(12), tag_name(13), tag_display_name(14), tag_type(15), tag_color(16), tag_icon(17), tag_use_count(18), tag_created_at(19)
                let tag_type_str: String = row.get(15)?;
                let tag_type = TagType::from_str(&tag_type_str);
                let tag_created_at_ts: i64 = row.get(19)?;
                let tag_created_at = DateTime::from_timestamp(tag_created_at_ts, 0)
                    .ok_or_else(|| AppError::Unknown(format!("无效的 tag_created_at 时间戳: {}", tag_created_at_ts)))?;

                let tag = Tag {
                    id: tag_id_opt,
                    name: row.get(13)?,
                    display_name: row.get(14)?,
                    tag_type,
                    color: row.get(16)?,
                    icon: row.get(17)?,
                    use_count: row.get(18)?,
                    created_at: tag_created_at,
                };
                file_map.get_mut(&file_id).unwrap().1.push(tag);
            }
        }
        Ok(())
    }

    /// 重新计算所有标签的使用计数
    /// 基于 file_tags 表的实际数据修正 use_count
    pub fn recalculate_tag_counts(&self) -> Result<()> {
        let conn = self.conn.lock();

        println!("[DEBUG] 开始重新计算标签使用计数...");

        // 先重置所有标签的 use_count 为 0
        conn.execute("UPDATE tags SET use_count = 0", [])?;

        // 统计每个标签的实际使用次数
        let mut stmt = conn.prepare(
            "SELECT tag_id, COUNT(*) as count FROM file_tags GROUP BY tag_id"
        )?;

        let mut rows = stmt.query([])?;
        let mut tag_counts: Vec<(i64, i64)> = Vec::new();

        while let Some(row) = rows.next()? {
            let tag_id: i64 = row.get(0)?;
            let count: i64 = row.get(1)?;
            tag_counts.push((tag_id, count));
        }

        // 更新每个标签的 use_count
        for i in 0..tag_counts.len() {
            let (tag_id, count) = tag_counts[i];
            conn.execute(
                "UPDATE tags SET use_count = ?1 WHERE id = ?2",
                params![count, tag_id],
            )?;
        }

        println!("[DEBUG] 重新计算完成，更新了 {} 个标签", tag_counts.len());

        Ok(())
    }
}
