# 文件标签管理系统

一个高效的系统级文件标签管理工具，通过自动标签生成和自定义标签相结合的方式，让用户无需浏览文件内容即可快速定位文件。

## 功能特性

- 🔍 **快速搜索** - 支持多关键字（AND/OR 逻辑）的全文搜索
- 🏷️ **智能标签** - 根据文件类型、大小、时间自动生成标签
- ✏️ **自定义标签** - 支持手动添加和管理自定义标签
- 📁 **文件监控** - 自动监控指定目录的文件变化
- 🎨 **美观界面** - 基于 React + TailwindCSS 的现代化 UI
- ⚡ **高性能** - 使用 Rust + Tauri 构建，体积小、启动快
- 🌐 **跨平台** - 支持 Windows、macOS、Linux

## 技术栈

### 后端
- **Rust** - 系统级编程语言
- **Tauri** - 跨平台桌面应用框架
- **SQLite** - 内嵌数据库（支持 FTS5 全文索引）
- **notify** - 文件系统监听库

### 前端
- **React 18** - UI 框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具
- **TailwindCSS** - 样式方案
- **Zustand** - 状态管理
- **lucide-react** - 图标库

## 项目结构

```
file-tag-manager/
├── src/                       # React 前端
│   ├── components/            # UI 组件
│   │   ├── SearchBar/        # 搜索栏
│   │   ├── TagPanel/         # 标签面板
│   │   └── FileList/         # 文件列表
│   ├── lib/                  # 工具库
│   │   ├── api.ts           # API 客户端
│   │   └── utils.ts         # 工具函数
│   ├── stores/               # 状态管理
│   ├── types/                # 类型定义
│   ├── App.tsx              # 主应用
│   └── main.tsx              # 入口文件
├── src-tauri/                # Rust 后端
│   ├── src/
│   │   ├── db/              # 数据库模块
│   │   ├── watcher/         # 文件监控
│   │   ├── tagger/          # 标签生成
│   │   ├── search/          # 搜索模块
│   │   ├── commands/        # Tauri 命令
│   │   └── error.rs         # 错误处理
│   └── Cargo.toml           # Rust 依赖
└── package.json             # Node 依赖
```

## 开发指南

### 环境要求

- **Node.js** >= 18.0
- **Rust** >= 1.70
- **pnpm** (推荐) 或 npm/yarn

### 安装依赖

```bash
# 安装 Node 依赖
npm install

# 安装 Rust 依赖（自动）
# cargo build 会自动下载 Rust 依赖
```

### 开发模式

```bash
npm run tauri:dev
```

### 构建

```bash
npm run tauri:build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录。

### 代码规范

本项目遵循 `DEVELOPMENT_GUIDELINES.md` 中定义的开发准则。

## 使用说明

### 添加监控目录

1. 点击应用中的"添加目录"按钮
2. 选择要监控的文件夹路径
3. 设置是否递归监控子目录
4. 系统将自动扫描并索引该目录中的文件

### 搜索文件

1. 在搜索框中输入关键字
2. 支持多个关键字，用空格分隔
3. 切换 AND/OR 运算符控制搜索逻辑
4. 按回车或点击搜索按钮执行搜索

### 管理标签

**系统标签**：根据文件特征自动生成，包括：
- 文件类型：图片、音频、视频、文本、二进制
- 文件大小：小文件（<10KB）、大文件（>100MB）
- 时间标签：今日文件、本周文件、本月文件
- 路径标签：下载、文档、桌面等

**自定义标签**：
1. 在文件列表中选择文件
2. 点击"添加标签"按钮
3. 输入标签名称并选择颜色
4. 保存后即可使用该标签筛选文件

## 配置文件

应用配置和数据存储位置：

| 操作系统 | 路径 |
|---------|------|
| Windows | `%APPDATA%\file-tag-manager\` |
| macOS | `~/Library/Application Support/file-tag-manager/` |
| Linux | `~/.config/file-tag-manager/` |

## 性能指标

| 指标 | 目标值 |
|-----|-------|
| 应用启动时间 | < 3 秒 |
| 文件索引速度 | > 1000 文件/秒 |
| 搜索响应时间 | < 100ms (10万文件内) |
| 内存占用 | < 200MB (10万文件) |
| 应用体积 | 4-12MB |

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！

---

**开发文档**：
- [项目开发准则](./DEVELOPMENT_GUIDELINES.md)
- [概要设计](./DESIGN_OVERVIEW.md)
- [详细设计](./DETAILED_DESIGN.md)
