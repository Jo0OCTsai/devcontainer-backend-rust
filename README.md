# todo-backend-rust

todo backend 的 **Rust 栈适配镜像**（dev-env-settings 仓库 `backend-rust/` 子模块）——与 `devcontainer-backend`
保持相同 API 面与角色分工（骨架阶段：health + OpenAPI schema），作为 rust 栈工具链接入的 dogfooding 载体
（stack.json 声明 / CI rust 分支 / 守卫注册表的真实首例）。

- 镜像关系与边界约定见模块 `AGENTS.md`
- 轻依赖（tiny_http + serde_json）：降低首例接入变量；todo CRUD 随主 backend 同步演进
