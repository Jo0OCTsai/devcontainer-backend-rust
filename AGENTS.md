# backend-rust/AGENTS.md

> 本文件是 todo-backend-**rust** 的 agent 工作指引。通用开发流程原则见主仓库根 `AGENTS.md`；
> 与 Python 版 `backend/` 为**栈适配镜像**关系——API 面与分层角色对齐，实现按 Rust 惯例。

## 项目概述

todo app 后端的 Rust 镜像（骨架阶段）：health 端点 + 静态 OpenAPI schema，内存存储占位。

## 技术栈

- Rust 2021；`tiny_http`（轻量 HTTP）+ `serde`/`serde_json`——刻意轻依赖，降低首例接入变量
- lint：`cargo fmt --check` + `cargo clippy -- -D warnings`（复杂度类 lint 随首例验证定版加入）
- 测试：`cargo test`（业务层纯函数表驱动）

## 环境与命令

```bash
# 同步依赖
cargo fetch

# 运行测试
cargo test

# 启动开发服务器（默认 0.0.0.0:8002；ADDR 可覆盖）
cargo run

# lint / format
cargo fmt --check
cargo clippy -- -D warnings
```

## 目录结构

```
backend-rust/
├── src/main.rs    # 入口层（对齐 backend 的 config/ 角色：tiny_http 装配与监听）
├── src/app.rs     # 业务层（对齐 apps/ 角色：纯函数路由 + 表驱动测试）
├── api/openapi.json  # 静态 schema（include_str! 编译期嵌入）
└── Cargo.toml
```

## 约定（镜像 backend 的分层边界）

- **依赖方向**：`main → app` 单向；`app` 模块禁止反向依赖入口层。将来多模块膨胀时
  **优先 crate 拆分**（Cargo workspace：crate 边界由编译器强制 pub 可见性 + 显式依赖声明，
  这是 Rust 的天然守卫），而非 path 级规则。
- API 形状（路径、JSON 字段、尾斜杠、405/404 语义）与 `backend/` 保持一致，漂移即 bug。
