# 测试指南

> **版本**：1.0.0
> **更新日期**：2026-02-27

---

## 一、测试概述

本项目采用 Rust 单元测试和 React 组件测试相结合的策略。

| 测试类型 | 框架 | 覆盖范围 |
|---------|-------|---------|
| Rust 单元测试 | 内置测试框架 | 核心逻辑、数据模型 |
| React 组件测试 | Vitest + Testing Library | UI 组件、工具函数 |

---

## 二、Rust 测试

### 2.1 运行测试

```bash
# 运行所有测试
cd src-tauri
cargo test

# 运行特定模块测试
cargo test db
cargo test tagger

# 显示测试输出
cargo test -- --nocapture

# 运行测试并生成覆盖率（需要安装 cargo-tarpaulin）
cargo tarpaulin --out Html
```

### 2.2 测试文件位置

```
src-tauri/src/
├── db/
│   └── tests.rs              # 数据库模块测试
├── tagger/
│   └── auto.rs               # 标签生成测试（内嵌）
├── watcher/
│   └── tests.rs              # 文件监控测试
└── search/
    └── tests.rs              # 搜索模块测试
```

### 2.3 测试命名规范

```rust
#[test]
fn test_模块_功能() {
    // 测试逻辑
}

#[test]
fn test_模块_功能_场景() {
    // 测试逻辑
}
```

### 2.4 编写测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_detection() {
        let file_type = FileType::from_extension("jpg");
        assert_eq!(file_type, FileType::Image);
    }

    #[test]
    #[serial_test::serial]  // 串行测试（需要 serial_test 依赖）
    fn test_database_operation() {
        // 测试数据库操作
    }
}
```

---

## 三、React 测试

### 3.1 运行测试

```bash
# 运行所有测试
npm test

# 监听模式（自动重新运行）
npm test -- --watch

# UI 模式（可视化测试结果）
npm run test:ui

# 生成覆盖率报告
npm run test:coverage
```

### 3.2 测试文件位置

```
src/
├── components/__tests__/
│   ├── SearchBar.test.tsx
│   ├── TagPanel.test.tsx
│   └── FileList.test.tsx
└── lib/__tests__/
    └── utils.test.ts
```

### 3.3 测试命名规范

```typescript
describe('组件名', () => {
  it('应该做什么', () => {
    // 测试逻辑
  });

  it('应该在什么情况下做什么', () => {
    // 测试逻辑
  });
});
```

### 3.4 编写测试示例

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import Component from './Component';

// Mock 依赖
vi.mock('../../stores/useStore', () => ({
  useStore: vi.fn(() => ({ ... })),
}));

describe('Component', () => {
  it('应该渲染正确', () => {
    render(<Component />);
    expect(screen.getByText('Expected Text')).toBeInTheDocument();
  });

  it('应该响应用户交互', async () => {
    const user = userEvent.setup();
    render(<Component />);

    const button = screen.getByRole('button');
    await user.click(button);

    // 验证状态变化
  });
});
```

---

## 四、测试覆盖率

### 4.1 生成覆盖率报告

```bash
# Rust 覆盖率
cargo tarpaulin --out Html

# React 覆盖率
npm run test:coverage
```

### 4.2 查看覆盖率

- **Rust**: 打开 `tarpaulin-report.html`
- **React**: 打开 `coverage/index.html`

### 4.3 覆盖率目标

| 模块 | 目标覆盖率 |
|-----|-----------|
| 核心逻辑 (db, tagger) | ≥ 80% |
| 组件测试 | ≥ 70% |
| 工具函数 | ≥ 90% |

---

## 五、测试最佳实践

### 5.1 Rust 测试

1. **隔离性**：每个测试应该独立运行
2. **可读性**：测试名称应该描述清楚测试的目的
3. **Mock**：使用 `#[cfg(test)]` 模块隔离测试代码
4. **串行化**：对于需要修改全局状态的测试，使用 `serial_test`

### 5.2 React 测试

1. **用户视角**：测试应该模拟真实用户操作
2. **避免实现细节**：不要测试内部状态，测试可见行为
3. **合理 Mock**：只 Mock 外部依赖，不要 Mock 被测组件
4. **异步测试**：使用 `await` 正确处理异步操作

---

## 六、常见问题

### Q1: Rust 测试报 "database locked"

A: 确保测试使用内存数据库 (`Connection::open_in_memory()`)。

### Q2: React 测试报 "Module not found"

A: 检查 `vite.config.ts` 中的路径别名配置是否正确。

### Q3: 测试超时

A: 增加测试超时时间：
```typescript
it('should do something', { timeout: 10000 }, () => {
  // 测试逻辑
});
```

---

## 七、相关资源

- [Vitest 文档](https://vitest.dev/)
- [Testing Library 文档](https://testing-library.com/)
- [Rust 测试指南](https://doc.rust-lang.org/book/ch11-00-testing.html)
