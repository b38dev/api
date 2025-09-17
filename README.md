# Axum 项目模板

这是一个使用 Axum 框架构建的 Rust Web 服务项目模板。

## 结构

- `src/main.rs`: 应用入口，负责初始化 tracing 和启动服务器。
- `src/lib.rs`: 库的根文件，用于组织应用逻辑和创建 Axum Router。
- `src/routes.rs`: 定义应用的 HTTP 路由。
- `src/handlers.rs`: 实现路由对应的处理函数（业务逻辑）。
- `src/models.rs`: 定义数据结构（例如用于 JSON payload 的结构体）。
- `src/error.rs`: 定义自定义错误类型和处理。
- `Cargo.toml`: 项目依赖和配置。

## 如何运行

1.  **安装 Rust**: 如果你还没有安装，请访问 rust-lang.org。

2.  **构建项目**:
    ```bash
    cargo build
    ```

3.  **运行项目**:
    ```bash
    cargo run
    ```
    服务将会在 `http://0.0.0.0:3000` 上启动。

## 测试端点

- **GET /onair**: `curl http://localhost:3000/onair`