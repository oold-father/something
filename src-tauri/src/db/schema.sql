-- =====================================================
-- 1. 文件表 (files)
-- =====================================================
CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    size INTEGER NOT NULL DEFAULT 0,
    file_type TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL,
    accessed_at INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    indexed_at INTEGER NOT NULL,
    metadata TEXT
);

CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
CREATE INDEX IF NOT EXISTS idx_files_type ON files(file_type);
CREATE INDEX IF NOT EXISTS idx_files_status ON files(status);
CREATE INDEX IF NOT EXISTS idx_files_created_at ON files(created_at DESC);


-- =====================================================
-- 2. 标签表 (tags)
-- =====================================================
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    tag_type TEXT NOT NULL CHECK(tag_type IN ('system', 'custom')),
    color TEXT NOT NULL DEFAULT '#007ACC',
    icon TEXT,
    use_count INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tags_type ON tags(tag_type);
CREATE INDEX IF NOT EXISTS idx_tags_use_count ON tags(use_count DESC);


-- =====================================================
-- 3. 文件标签关联表 (file_tags)
-- =====================================================
CREATE TABLE IF NOT EXISTS file_tags (
    file_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    is_auto INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (file_id, tag_id),
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_file_tags_file_id ON file_tags(file_id);
CREATE INDEX IF NOT EXISTS idx_file_tags_tag_id ON file_tags(tag_id);
CREATE INDEX IF NOT EXISTS idx_file_tags_is_auto ON file_tags(is_auto);


-- =====================================================
-- 4. 监控目录表 (watched_directories)
-- =====================================================
CREATE TABLE IF NOT EXISTS watched_directories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    recursive INTEGER NOT NULL DEFAULT 1,
    filters TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    last_scanned_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_watched_enabled ON watched_directories(enabled);


-- =====================================================
-- 5. 全文搜索索引 (file_tags_fts)
-- =====================================================
CREATE VIRTUAL TABLE IF NOT EXISTS file_tags_content USING fts5(
    file_id,
    file_name,
    file_path,
    tag_names
);

-- 触发器：同步数据到 FTS 索引
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


-- =====================================================
-- 6. 系统配置表 (settings)
-- =====================================================
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 默认配置
INSERT OR IGNORE INTO settings (key, value, updated_at)
VALUES ('max_index_files', '1000000', strftime('%s', 'now'));

INSERT OR IGNORE INTO settings (key, value, updated_at)
VALUES ('search_result_limit', '100', strftime('%s', 'now'));
