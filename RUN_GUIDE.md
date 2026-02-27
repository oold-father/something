# 运行指南

> **项目**：文件标签管理系统
> **版本**：v0.1.0

---

## 环境要求

| 组件 | 要求 | 当前状态 |
|-----|------|---------|
| Node.js | ≥ 18.0 | ✅ 已安装 |
| Tauri CLI | 2.10.0 | ✅ 已安装 |
| Rust | ≥ 1.70 | ❌ 未安装 |

**重要**：Tauri 应用需要 Rust 才能编译运行。

---

## 安装 Rust

### Windows

**方法 1：使用官方安装程序（推荐）**

1. 访问：https://rustup.rs/
2. 下载并运行 `rustup-init.exe`
3. 按照提示安装
4. 重启终端，执行：`rustup default stable`

**方法 2：使用 winget（Windows 10+）**

```powershell
winget install Rustlang.Rustup
```

**验证安装：**
```bash
cargo --version
# 应显示类似：cargo 1.70.0
```

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## 运行开发模式

安装 Rust 后，执行：

```bash
# 开发模式（带热重载）
npm run tauri:dev

# 或者
npx tauri dev
```

首次运行会：
1. 编译 Rust 代码（较慢）
2. 启动 Vite 开发服务器
3. 打开应用窗口

---

## 构建发布版本

```bash
# 构建（生成可执行文件）
npm run tauri:build

# 构建产物位置
# Windows: src-tauri/target/release/bundle/
```

---

## 当前项目状态

### 已完成功能
- ✅ 项目骨架搭建
- ✅ 数据库模块
- ✅ 文件监控模块
- ✅ 自动标签生成
- ✅ 标签管理功能
- ✅ 搜索功能
- ✅ 基础前端界面
- ✅ 测试框架搭建

### 已知限制
- ⚠️ Rust 环境未安装，无法编译运行
- ⚠️ React 组件 Mock 需要完善
- ⚠️ E2E 测试未添加
- ⚠️ 设置面板未实现

---

## 下一步

1. 安装 Rust 开发环境
2. 运行 `npm run tauri:dev`
3. 查看第一版程序运行效果
4. 根据使用反馈进行优化
