# 文件标签管理系统 - 详细设计文档

> **设计版本**：2.0.0
> **创建日期**：2026-02-27
> **状态**：已审核

---

## 一、设计背景

本系统旨在为个人电脑用户提供一个高效的文件标签管理工具，解决文件数量庞大导致查找困难的问题。系统通过自动标签生成和自定义标签相结合的方式，让用户无需浏览文件内容即可快速定位文件。

**设计约束**：
- 跨平台支持：Windows / macOS / Linux
- 标签系统：扁平结构（无层级关系）
- 内容分析：仅元数据（扩展名、大小、时间）
- 性能目标：索引 > 1000 文件/秒，搜索 < 100ms

---

## 二、技术栈选型

### 2.1 技术组合

| 层级 | 技术选型 | 版本要求 | 理由 |
|-----|---------|---------|------|
| 桌面框架 | **Tauri** | 2.x | 体积小（4-12MB）、内存占用低（50-100MB）、Rust后端高性能 |
| 后端语言 | **Rust** | 1.70+ | 零成本抽象、内存安全、并发性能优异 |
| 前端语言 | **TypeScript** | 5.x | 类型安全、开发体验好 |
| 前端框架 | **React** | 18.x | 生态成熟、组件化开发 |
| 构建工具 | **Vite** | 5.x | 快速开发体验 |
| UI框架 | **shadcn/ui** | latest | 基于 Radix UI，组件丰富且可定制 |
| 样式方案 | **TailwindCSS** | 3.x | 原子化 CSS，快速开发 |
| 状态管理 | **Zustand** | 4.x | 轻量级、简单易用 |
| 数据请求 | **React Query** | 5.x | 自动缓存、重试机制 |
| 数据库 | **SQLite** | 3.40+ | 零配置、FTS5全文索引 |
| 文件监控 | **notify** | 6.x | Rust 跨平台文件系统监听 |

### 2.2 目录结构

```
file-tag-manager/
├── src-tauri/                 # Rust 后端
│   ├── src/
│   │   ├── main.rs            # 应用入口
│   │   ├── lib.rs             # 库导出
│   │   ├── db/                # 数据库模块
│   │   │   ├── mod.rs
│   │   │   ├── models.rs      # 数据模型
│   │   │   ├── schema.sql     # 数据库表结构
│   │   │   └── queries.rs     # 数据库操作
│   │   ├── watcher/           # 文件监控模块
│   │   │   ├── mod.rs
│   │   │   └── event.rs       # 事件处理
│   │   ├── tagger/            # 标签生成模块
│   │   │   ├── mod.rs
│   │   │   └── auto.rs        # 自动标签规则
│   │   ├── search/            # 搜索模块
│   │   │   ├── mod.rs
│   │   │   └── index.rs       # 搜索索引
│   │   ├── commands/          # Tauri 命令
│   │   │   ├── mod.rs
│   │   │   ├── file.rs
│   │   │   ├── tag.rs
│   │   │   └── search.rs
│   │   └── utils/             # 工具函数
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── src/                       # React 前端
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/            # 组件
│   │   ├── FileList/
│   │   ├── TagPanel/
│   │   ├── SearchBar/
│   │   └── Settings/
│   ├── hooks/                 # 自定义 Hooks
│   ├── stores/                # Zustand 状态
│   ├── lib/                   # 工具函数
│   └── types/                 # TypeScript 类型
├── package.json
└── README.md
```

---

## 三、详细数据模型设计

### 3.1 数据库表结构

```sql
-- =====================================================
-- 1. 文件表 (files)
-- =====================================================
CREATE TABLE IF NOT EXISTS files (
    -- 主键
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    -- 文件路径（去重索引）
    path TEXT NOT NULL UNIQUE,

    -- 文件基本信息
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    size INTEGER NOT NULL DEFAULT 0,
    file_type TEXT NOT NULL,  -- 'image', 'audio', 'video', 'text', 'binary', 'other'

    -- 时间戳
    created_at INTEGER NOT NULL,
    modified_at INTEGER NOT NULL,
    accessed_at INTEGER NOT NULL,

    -- 系统状态
    status TEXT NOT NULL DEFAULT 'active',  -- 'active', 'deleted', 'moved'
    indexed_at INTEGER NOT NULL,

    -- 扩展字段（JSON 存储，用于未来扩展）
    metadata TEXT  -- JSON: {resolution: "1920x1080", duration: 180, etc.}
);

CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_files_type ON files(file_type);
CREATE INDEX idx_files_status ON files(status);
CREATE INDEX idx_files_created_at ON files(created_at DESC);


-- =====================================================
-- 2. 标签表 (tags)
-- =====================================================
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,  -- 显示名称（支持国际化）
    tag_type TEXT NOT NULL CHECK(tag_type IN ('system', 'custom')),
    color TEXT NOT NULL DEFAULT '#007ACC',  -- 显示颜色
    icon TEXT,  -- 图标标识
    use_count INTEGER NOT NULL DEFAULT 0,  -- 使用次数
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_tags_type ON tags(tag_type);
CREATE INDEX idx_tags_use_count ON tags(use_count DESC);


-- =====================================================
-- 3. 文件标签关联表 (file_tags)
-- =====================================================
CREATE TABLE IF NOT EXISTS file_tags (
    file_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    is_auto INTEGER NOT NULL DEFAULT 0,  -- 是否为自动标签：0=自定义，1=自动
    created_at INTEGER NOT NULL,
    PRIMARY KEY (file_id, tag_id),
    FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX idx_file_tags_file_id ON file_tags(file_id);
CREATE INDEX idx_file_tags_tag_id ON file_tags(tag_id);
CREATE INDEX idx_file_tags_is_auto ON file_tags(is_auto);


-- =====================================================
-- 4. 监控目录表 (watched_directories)
-- =====================================================
CREATE TABLE IF NOT EXISTS watched_directories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    recursive INTEGER NOT NULL DEFAULT 1,  -- 是否递归监控
    filters TEXT,  -- JSON: {extensions: ['jpg','png'], exclude: ['*.tmp']}
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    last_scanned_at INTEGER
);

CREATE INDEX idx_watched_enabled ON watched_directories(enabled);


-- =====================================================
-- 5. 搜索历史表 (search_history)
-- =====================================================
CREATE TABLE IF NOT EXISTS search_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    query TEXT NOT NULL,
    result_count INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_search_history_created_at ON search_history(created_at DESC);


-- =====================================================
-- 6. 全文搜索索引 (file_tags_fts)
-- =====================================================
CREATE VIRTUAL TABLE IF NOT EXISTS file_tags_fts USING fts5(
    file_id,
    file_name,
    file_path,
    tag_names,  -- 逗号分隔的标签名称
    content='file_tags_content',
    contentless_delete=1
);

-- FTS 内容表
CREATE TABLE IF NOT EXISTS file_tags_content (
    file_id INTEGER PRIMARY KEY,
    file_name TEXT,
    file_path TEXT,
    tag_names TEXT
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
-- 7. 系统配置表 (settings)
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
```

### 3.2 ER 图

```
┌─────────────────┐         ┌─────────────────┐         ┌─────────────────┐
│     files       │◄───────►│   file_tags     │◄───────►│     tags        │
│─────────────────│         │─────────────────│         │─────────────────│
│ id (PK)         │         │ file_id (PK,FK) │         │ id (PK)         │
│ path (UNIQUE)   │         │ tag_id (PK,FK)  │         │ name (UNIQUE)   │
│ name            │         │ is_auto         │         │ display_name    │
│ extension       │         │ created_at       │         │ tag_type        │
│ size            │         └─────────────────┘         │ color           │
│ file_type       │                                      │ use_count       │
│ created_at      │         ┌─────────────────┐         └─────────────────┘
│ modified_at     │         │watched_director │
│ status          │         │      ies        │
│ metadata        │         │─────────────────│
└─────────────────┘         │ id (PK)         │
                            │ path (UNIQUE)   │
┌─────────────────┐         │ recursive       │
│ search_history  │         │ enabled         │
│─────────────────│         └─────────────────┘
│ id (PK)         │
│ query           │         ┌─────────────────┐
│ result_count    │         │    settings     │
│ created_at      │         │─────────────────│
└─────────────────┘         │ key (PK)        │
                            │ value           │
                            │ updated_at      │
                            └─────────────────┘
```

### 3.3 Rust 数据模型

```rust
// src-tauri/src/db/models.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Image,
    Audio,
    Video,
    Text,
    Binary,
    Other,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" => FileType::Image,
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => FileType::Audio,
            "mp4" | "avi" | "mkv" | "mov" | "webm" | "flv" => FileType::Video,
            "txt" | "md" | "json" | "xml" | "yaml" | "csv" | "log" => FileType::Text,
            "exe" | "dll" | "so" | "bin" | "zip" | "rar" | "7z" => FileType::Binary,
            _ => FileType::Other,
        }
    }
}

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
    pub status: String,
    pub indexed_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TagType {
    System,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTag {
    pub file_id: i64,
    pub tag_id: i64,
    pub is_auto: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedDirectory {
    pub id: Option<i64>,
    pub path: String,
    pub recursive: bool,
    pub filters: Option<serde_json::Value>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_scanned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: File,
    pub tags: Vec<Tag>,
    pub relevance: f32,
}
```

---

## 四、核心模块详细设计

### 4.1 文件监控模块 (watcher)

#### 4.1.1 模块职责
- 监控指定目录的文件系统事件（创建、修改、删除、移动）
- 将事件放入队列，异步处理
- 处理文件变更的标签更新

#### 4.1.2 组件结构

```
src-tauri/src/watcher/
├── mod.rs           # 模块入口
├── event.rs         # 事件定义和处理器
├── queue.rs         # 事件队列
└── scanner.rs       # 初始扫描器
```

#### 4.1.3 事件流程

```
文件系统事件 → Notify 事件 → 事件队列 → 处理器 → 数据库更新
                      ↓                    ↓
                  去重/合并              标签生成
```

#### 4.1.4 关键代码结构

```rust
// src-tauri/src/watcher/mod.rs

use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::mpsc;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
    event_sender: mpsc::Sender<FileEvent>,
    watched_paths: HashSet<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created { path: PathBuf },
    Modified { path: PathBuf },
    Deleted { path: PathBuf },
    Moved { from: PathBuf, to: PathBuf },
}

impl FileWatcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel(1000);
        let watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                // 将 notify 事件转换为自定义 FileEvent
            }
        })?;
        Ok(Self { watcher, event_sender: tx, watched_paths: HashSet::new() })
    }

    pub fn watch(&mut self, path: &Path, recursive: bool) -> Result<()> {
        let mode = if recursive { RecursiveMode::Recursive } else { RecursiveMode::NonRecursive };
        self.watcher.watch(path, mode)?;
        self.watched_paths.insert(path.to_path_buf());
        Ok(())
    }
}
```

### 4.2 自动标签生成模块 (tagger)

#### 4.2.1 模块职责
- 根据文件元数据生成自动标签
- 支持扩展名、文件类型、大小等规则
- 可扩展的规则引擎

#### 4.2.2 标签生成规则

| 规则类型 | 规则条件 | 生成的标签 |
|---------|---------|-----------|
| 文件类型 | extension in ['jpg','png','gif'] | 图片 |
| 文件类型 | extension in ['mp3','wav','flac'] | 音频 |
| 文件类型 | extension in ['mp4','avi','mkv'] | 视频 |
| 文件类型 | extension in ['txt','md','log'] | 文本 |
| 文件大小 | size < 1024 | 小文件 |
| 文件大小 | size > 100 * 1024 * 1024 | 大文件 |
| 日期 | created_at in 今天 | 今日文件 |
| 日期 | created_at in 本周 | 本周文件 |
| 路径 | path contains 'Downloads' | 下载 |

#### 4.2.3 关键代码结构

```rust
// src-tauri/src/tagger/mod.rs

use crate::db::models::{File, Tag, TagType};

pub struct AutoTagger {
    rules: Vec<TagRule>,
}

#[derive(Debug, Clone)]
pub struct TagRule {
    pub name: String,
    pub condition: TagCondition,
}

pub enum TagCondition {
    FileType(Vec<String>),           // 文件类型匹配
    FileSize { min: Option<u64>, max: Option<u64> },  // 文件大小范围
    DatePattern(DatePattern),         // 日期模式
    PathContains(String),            // 路径包含
    Extension(Vec<String>),          // 扩展名匹配
}

pub enum DatePattern {
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
}

impl AutoTagger {
    pub fn new() -> Self {
        Self { rules: Self::default_rules() }
    }

    pub fn generate_tags(&self, file: &File) -> Vec<String> {
        self.rules
            .iter()
            .filter(|rule| self.match_rule(rule, file))
            .map(|rule| rule.name.clone())
            .collect()
    }

    fn default_rules() -> Vec<TagRule> {
        vec![
            TagRule {
                name: "图片".to_string(),
                condition: TagCondition::FileType(vec!["image".to_string()]),
            },
            TagRule {
                name: "音频".to_string(),
                condition: TagCondition::FileType(vec!["audio".to_string()]),
            },
            // ... 更多默认规则
        ]
    }
}
```

### 4.3 标签管理模块 (tags)

#### 4.3.1 模块职责
- 标签的增删改查
- 文件与标签的关联管理
- 标签使用统计

#### 4.3.2 关键接口

```rust
// src-tauri/src/commands/tag.rs

#[tauri::command]
pub async fn get_all_tags() -> Result<Vec<Tag>, String> { /* ... */ }

#[tauri::command]
pub async fn get_tags_by_file(file_id: i64) -> Result<Vec<Tag>, String> { /* ... */ }

#[tauri::command]
pub async fn create_tag(name: String, display_name: String, color: String) -> Result<i64, String> { /* ... */ }

#[tauri::command]
pub async fn add_tag_to_file(file_id: i64, tag_name: String) -> Result<(), String> { /* ... */ }

#[tauri::command]
pub async fn remove_tag_from_file(file_id: i64, tag_id: i64) -> Result<(), String> { /* ... */ }

#[tauri::command]
pub async fn batch_add_tags(file_ids: Vec<i64>, tag_names: Vec<String>) -> Result<(), String> { /* ... */ }

#[tauri::command]
pub async fn get_files_by_tags(tag_names: Vec<String>) -> Result<Vec<File>, String> { /* ... */ }
```

### 4.4 搜索模块 (search)

#### 4.4.1 模块职责
- 基于 FTS5 全文索引的快速搜索
- 支持多关键字组合（AND/OR 逻辑）
- 搜索结果排序（相关度、时间）

#### 4.4.2 搜索流程

```
用户输入 → 分词处理 → FTS 查询 → 结果聚合 → 排序 → 返回
           ↓
    关键字提取（支持中文分词）
           ↓
    查询优化（使用索引）
```

#### 4.4.3 关键代码结构

```rust
// src-tauri/src/search/index.rs

use rusqlite::Connection;

pub struct SearchEngine {
    conn: Arc<Mutex<Connection>>,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub keywords: Vec<String>,
    pub operator: SearchOperator,
    pub file_type_filter: Option<FileType>,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchOperator {
    And,  // 所有关键字都匹配
    Or,   // 任一关键字匹配
}

impl SearchEngine {
    pub async fn search(&self, query: SearchQuery) -> Result<Vec<SearchResult>> {
        let conn = self.conn.lock().await;

        // 构建 FTS 查询
        let fts_query = self.build_fts_query(&query)?;

        // 执行搜索
        let mut stmt = conn.prepare_cached(&format!(
            "SELECT f.file_id, f.name, f.path, f.extension, f.size, f.file_type,
                    f.created_at, f.modified_at, fts.rank
             FROM file_tags_fts fts
             JOIN files f ON f.file_id = f.id
             WHERE file_tags_fts MATCH ?1
             AND f.status = 'active'
             ORDER BY fts.rank DESC, f.created_at DESC
             LIMIT ?2 OFFSET ?3",
            fts_query, query.limit, query.offset
        ))?;

        let results = stmt.query_map(params![fts_query], |row| {
            // 解析结果
        })?;

        Ok(results.collect())
    }

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
                    .map(|t| format!("({} OR {})", t, t))
                    .collect();
                Ok(terms.join(" OR "))
            }
        }
    }
}
```

### 4.5 缓存管理模块 (cache)

#### 4.5.1 缓存策略

| 缓存项 | 缓存时间 | 清理策略 |
|-------|---------|---------|
| 搜索结果 | 5 分钟 | LRU |
| 标签列表 | 10 分钟 | 定期刷新 |
| 文件元数据 | 会话期 | 关闭时清理 |
| 索引状态 | 实时 | 事件驱动更新 |

---

## 五、API 接口设计

### 5.1 Tauri Commands API

```typescript
// src/types/api.ts

// ===== 文件相关 =====
export interface File {
  id?: number;
  path: string;
  name: string;
  extension: string;
  size: number;
  fileType: 'image' | 'audio' | 'video' | 'text' | 'binary' | 'other';
  createdAt: string;
  modifiedAt: string;
  status: 'active' | 'deleted' | 'moved';
}

export interface FileWithTags extends File {
  tags: Tag[];
}

// ===== 标签相关 =====
export interface Tag {
  id?: number;
  name: string;
  displayName: string;
  tagType: 'system' | 'custom';
  color: string;
  icon?: string;
  useCount: number;
}

// ===== 搜索相关 =====
export interface SearchQuery {
  keywords: string[];
  operator: 'AND' | 'OR';
  fileTypeFilter?: string;
  limit: number;
  offset: number;
}

export interface SearchResult {
  file: File;
  tags: Tag[];
  relevance: number;
}

export interface SearchResultResponse {
  results: SearchResult[];
  total: number;
}

// ===== 监控目录相关 =====
export interface WatchedDirectory {
  id?: number;
  path: string;
  recursive: boolean;
  filters?: DirectoryFilters;
  enabled: boolean;
}

export interface DirectoryFilters {
  extensions?: string[];
  exclude?: string[];
}

// ===== API 调用 =====
export const api = {
  // 文件操作
  getFiles: (params?: { limit?: number; offset?: number }) => Promise<File[]>,
  getFileById: (id: number) => Promise<FileWithTags>,
  addFiles: (paths: string[]) => Promise<void>,

  // 标签操作
  getAllTags: () => Promise<Tag[]>,
  getTagsByFile: (fileId: number) => Promise<Tag[]>,
  createTag: (name: string, displayName: string, color: string) => Promise<number>,
  addTagToFile: (fileId: number, tagName: string) => Promise<void>,
  removeTagFromFile: (fileId: number, tagId: number) => Promise<void>,
  batchAddTags: (fileIds: number[], tagNames: string[]) => Promise<void>,
  deleteTag: (tagId: number) => Promise<void>,

  // 搜索操作
  search: (query: SearchQuery) => Promise<SearchResultResponse>,

  // 监控目录操作
  getWatchedDirectories: () => Promise<WatchedDirectory[]>,
  addWatchedDirectory: (path: string, recursive: boolean, filters?: DirectoryFilters) => Promise<void>,
  removeWatchedDirectory: (id: number) => Promise<void>,

  // 系统操作
  getStats: () => Promise<SystemStats>,
  startIndexing: () => Promise<void>,
  stopIndexing: () => Promise<void>,
};

export interface SystemStats {
  totalFiles: number;
  indexedFiles: number;
  totalTags: number;
  watchedDirectories: number;
}
```

---

## 六、性能优化方案

### 6.1 索引优化

| 优化项 | 实现方式 | 预期效果 |
|-------|---------|---------|
| FTS5 全文索引 | 使用 FTS5 虚拟表 | 搜索 < 100ms |
| 复合索引 | (file_type, status) | 类型查询加速 |
| 批量插入 | 使用事务 | 10x 写入性能 |
| 延迟索引 | 后台异步处理 | 不阻塞 UI |

### 6.2 搜索优化

```
搜索优化策略：
1. 关键字预处理 - 去除停用词、统一大小写
2. 查询缓存 - 相同查询直接返回缓存结果
3. 分页加载 - 每次只加载 50-100 条
4. 结果预加载 - 预加载下一页
5. 增量更新 - 只更新变更的索引
```

### 6.3 并发处理

```rust
// 使用多线程处理文件索引
use tokio::task;

pub async fn batch_index_files(paths: Vec<PathBuf>) -> Result<()> {
    let tasks: Vec<_> = paths
        .chunks(100)  // 每批 100 个文件
        .map(|chunk| {
            let chunk = chunk.to_vec();
            task::spawn_blocking(move || {
                chunk.iter().for_each(|path| index_file(path));
            })
        })
        .collect();

    for task in tasks {
        task.await??;
    }
    Ok(())
}
```

---

## 七、错误处理与容错设计

### 7.1 错误类型

```rust
// src-tauri/src/error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("文件不存在: {0}")]
    FileNotFound(String),

    #[error("标签不存在: {0}")]
    TagNotFound(String),

    #[error("文件已监控: {0}")]
    AlreadyWatched(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

### 7.2 恢复机制

| 场景 | 恢复策略 |
|-----|---------|
| 文件访问失败 | 跳过并记录日志，定期重试 |
| 数据库损坏 | 备份恢复 + 重建索引 |
| 监控中断 | 重新注册监听器 |
| 磁盘空间不足 | 暂停索引，提示用户 |

---

## 八、部署与配置

### 8.1 数据存储位置

| 操作系统 | 数据路径 |
|---------|---------|
| Windows | `%APPDATA%\file-tag-manager\` |
| macOS | `~/Library/Application Support/file-tag-manager/` |
| Linux | `~/.config/file-tag-manager/` |

### 8.2 配置文件

```json
// config.json
{
  "database": {
    "path": "file_tags.db",
    "vacuum_interval": 86400
  },
  "watcher": {
    "max_watchers": 100,
    "debounce_ms": 500
  },
  "indexing": {
    "batch_size": 100,
    "max_workers": 4
  },
  "search": {
    "result_limit": 100,
    "cache_ttl": 300
  }
}
```

---

## 九、开发计划

### 9.1 阶段划分

| 阶段 | 任务 | 产出 | 工作量 |
|-----|------|------|-------|
| **Phase 1: 基础架构** | 项目初始化、数据库设计、文件监控 | 可运行的项目骨架 | 30% |
| **Phase 2: 核心功能** | 自动标签生成、标签管理 | 完整的标签功能 | 25% |
| **Phase 3: 搜索与 UI** | 搜索引擎、前端界面 | 可用的完整应用 | 30% |
| **Phase 4: 测试与优化** | 单元测试、E2E 测试、性能优化 | 稳定发布版本 | 15% |

### 9.2 关键里程碑

- **M1** (Phase 1 完成后): 文件监控和数据库初始化完成
- **M2** (Phase 2 完成后): 标签自动生成和管理完成
- **M3** (Phase 3 完成后): 搜索功能和基础 UI 完成
- **M4** (Phase 4 完成后): 完整功能发布

---

## 十、验证测试计划

### 10.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension("jpg"), FileType::Image);
        assert_eq!(FileType::from_extension("mp3"), FileType::Audio);
    }

    #[test]
    fn test_tag_generation() {
        let file = File {
            extension: "jpg".to_string(),
            file_type: FileType::Image,
            size: 1024 * 1024,
            // ...
        };
        let tags = AutoTagger::new().generate_tags(&file);
        assert!(tags.contains(&"图片".to_string()));
    }
}
```

### 10.2 集成测试

- 文件监控 → 数据库写入 → 标签生成 流程测试
- 搜索功能端到端测试
- 批量操作测试

### 10.3 性能测试

| 测试项 | 测试方法 | 目标值 |
|-------|---------|-------|
| 索引速度 | 索引 10,000 个文件 | > 1000 文件/秒 |
| 搜索响应 | 10 万文件中搜索 | < 100ms |
| 内存占用 | 10 万文件运行 | < 200MB |

---

## 十一、附录

### 11.1 依赖清单

**Cargo.toml (Rust)**:
```toml
[dependencies]
tauri = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.30", features = ["bundled"] }
notify = "6"
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1"
```

**package.json (Node)**:
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2.0.0",
    "zustand": "^4.4.0",
    "@tanstack/react-query": "^5.0.0"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "vite": "^5.0.0",
    "tailwindcss": "^3.3.0"
  }
}
```

---

**设计文档版本**: 2.0.0
**最后更新**: 2026-02-27
