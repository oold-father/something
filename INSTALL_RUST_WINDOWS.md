# Rust 安装指南 (Windows)

---

## 方法 1：官方安装程序（推荐）

### 步骤

1. 下载安装程序
   访问：https://www.rust-lang.org/tools/install

2. 运行安装程序
   下载 `rustup-init.exe` 并运行
   按照提示操作

3. 重启终端
   关闭并重新打开 PowerShell 或 CMD

4. 验证安装
   ```powershell
   cargo --version
   ```

---

## 方法 2：使用 winget (Windows 10/11)

```powershell
# 安装 Rust
winget install Rustlang.Rustup

# 设置默认工具链
rustup default stable

# 添加到 PATH（可能需要重启终端）
```

---

## 方法 3：使用 Chocolatey

```powershell
# 安装 Rust（如果已安装 chocolatey）
choco install rustup

# 安装 Rust
rustup default stable
```

---

## 验证安装

安装完成后，在新的终端窗口执行：

```bash
cargo --version
# 应该显示类似：cargo 1.70.0 或更新版本

rustc --version
# 应该显示：rustc 1.70.0 或更新版本
```

---

## 运行项目

安装 Rust 后，在项目根目录执行：

```bash
# 开发模式（带热重载）
npm run tauri:dev

# 构建生产版本
npm run tauri:build
```

---

## 故障排除

### 问题：`cargo: command not found`

**原因**：PATH 环境变量未正确设置

**解决方法**：

1. 检查 Rust 安装路径
   ```powershell
   $env:USERPROFILE\.cargo\bin
   ```

2. 添加到 PATH（临时，当前会话有效）
   ```powershell
   $env:PATH = "$env:PATH;$env:USERPROFILE\.cargo\bin"
   ```

3. 添加到 PATH（永久）
   - 系统设置 → 环境变量 → 系统变量 → Path
   - 添加：`%USERPROFILE%\.cargo\bin`

4. 重启终端

### 问题：`command not found: winget`

**原因**：Windows App Installer 未安装

**解决方法**：
- 访问 Microsoft Store 搜索 "App Installer"
- 或下载 Microsoft Store 版本

---

## 环境变量位置

| 变量 | 值 |
|-------|-----|
| CARGO_HOME | `%USERPROFILE%\.cargo` |
| RUSTUP_HOME | `%USERPROFILE%\.rustup` |
| PATH（需添加） | `%USERPROFILE%\.cargo\bin` |

---

安装 Rust 后，重新在终端运行 `cargo --version` 验证！
