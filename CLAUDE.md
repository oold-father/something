# 项目规范

## 语言规范
- 请始终使用简体中文与我对话。
- 所有生成的解释、文档和注释都请使用中文，但函数名和关键词保持英文。

## 项目概述

**something** 是一个系统级文件标签管理桌面应用，基于 Tauri 2.x 构建，为本地文件提供自动标签生成、自定义标签和全文搜索功能。

### 技术栈

| 层级 | 技术 | 版本 |
|-----|------|------|
| 桌面框架 | Tauri | 2.x |
| 后端语言 | Rust | 1.70+ |
| 数据库 | SQLite (rusqlite, bundled) | 0.30 |
| 文件监控 | notify | 6.x |
| 前端框架 | React + TypeScript | 18.x / 5.x |
| 构建工具 | Vite | 5.x |
| 样式方案 | TailwindCSS | 3.x |
| 状态管理 | Zustand | 4.x |
| 数据请求 | @tanstack/react-query | 5.x |
| 图标库 | lucide-react | 0.300+ |

### 项目结构

```
something/
├── src/                          # React 前端
│   ├── components/               # UI 组件
│   │   ├── SearchBar/            # 搜索栏
│   │   ├── TagPanel/             # 标签面板
│   │   ├── FileList/             # 文件列表
│   │   ├── WatchedDirectories/   # 监控目录管理
│   │   ├── SettingsPanel/        # 设置面板
│   │   ├── AddTagModal/          # 添加标签弹窗
│   │   ├── Toast/                # 通知组件
│   │   └── __tests__/            # 组件测试
│   ├── stores/useStore.ts        # Zustand 全局状态
│   ├── lib/api.ts                # Tauri API 客户端
│   ├── lib/utils.ts              # 工具函数
│   ├── types/api.ts              # TypeScript 类型定义
│   └── App.tsx                   # 主应用
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── commands/             # Tauri 命令
│   │   │   ├── file.rs           # 文件操作命令
│   │   │   ├── tag.rs            # 标签操作命令
│   │   │   ├── search.rs         # 搜索命令
│   │   │   └── directory_watcher.rs  # 监控目录命令
│   │   ├── db/                   # 数据库模块
│   │   │   ├── models.rs         # 数据模型
│   │   │   ├── queries.rs        # 数据库操作
│   │   │   └── schema.sql        # 表结构
│   │   ├── watcher/              # 文件监控
│   │   │   ├── mod.rs            # 监控器
│   │   │   ├── event.rs          # 事件处理
│   │   │   ├── queue.rs          # 事件队列
│   │   │   └── scanner.rs        # 目录扫描器
│   │   ├── tagger/               # 标签生成
│   │   │   ├── auto.rs           # 自动标签规则
│   │   │   └── rules.rs          # 标签规则定义
│   │   ├── search/               # 搜索引擎
│   │   │   └── index.rs          # FTS5 全文索引
│   │   └── error.rs              # 错误类型
│   └── Cargo.toml
└── package.json
```

### 常用命令

```bash
npm run tauri:dev        # 开发模式（Rust + Vite 热重载）
npm run tauri:build      # 构建发布版本
npm run test             # 运行前端测试 (vitest)
npm run test:rust        # 运行 Rust 测试 (cargo test)
npm run test:coverage    # 前端测试覆盖率
```

### 数据库

使用 SQLite，表包括：files、tags、file_tags、watched_directories、search_history、settings，以及 FTS5 全文搜索索引 file_tags_fts。详细表结构见 `src-tauri/src/db/schema.sql`。

### Tauri 命令

注册在 `src-tauri/src/commands/mod.rs` 中，涵盖：
- **标签**：get_all_tags, get_tags_by_file, create_tag, add_tag_to_file, remove_tag_from_file, batch_add_tags, get_files_by_tags, delete_tag, update_tag, fix_tag_counts
- **文件**：get_files, get_file_by_id, get_file_by_path, add_file, delete_file, get_stats
- **搜索**：search_files
- **监控目录**：get_watched_directories, add_watched_directory, remove_watched_directory, update_watched_directory, scan_directory, scan_all_directories

## 团队协作开发规范

### 迭代规划
- 每次迭代开启时，根据接收到的需求列表生成本次迭代的所有任务
- 使用 TaskCreate 创建所有待开发任务，并记录到任务列表
- 使用 TaskUpdate 维护任务状态和依赖关系

### 开发流程
- 严格按照需求顺序开发，一个接一个进行
- 任务开发完成后，编译并启动应用进行验收
- 验收通过后才能开发下一个任务
- 不验收不得开始下一个任务

### 任务分配
- 任务分配时，通过 SendMessage 工具通知对应团队成员
- 状态更新使用 TaskUpdate 工具
- 完成后清理已完成的任务

### 任务状态
- pending: 待分配
- in_progress: 进行中
- completed: 已完成
- deleted: 已删除

### 开发角色
- frontend-coder: 负责前端组件开发
- backend-coder: 负责后端API和数据库操作
- devops: 负责问题定界和定位
- tester: 负责测试验证和代码审计
- planner: 负责任务规划和架构设计
