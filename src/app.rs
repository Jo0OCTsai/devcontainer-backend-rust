//! 业务层（对齐 backend 的 apps/ 角色）：路由分发与响应构造。
//! 边界约定：本模块不反向依赖入口层（src/main.rs）；将来 crate 拆分时边界由 Cargo workspace 编译器强制。

use serde::Serialize;

/// 对齐 backend HealthResponseSerializer 的字段形状（db 为内存存储连通性占位，恒 ok）。
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub db: String,
}

/// 处理结果（状态码 + Content-Type + Body）；入口层只负责 IO 装配。
pub struct Response {
    pub status: u16,
    pub content_type: &'static str,
    pub body: String,
}

/// 处理一个请求（方法 + 路径）。纯函数，表驱动测试直测。
pub fn handle(method: &str, path: &str) -> Response {
    let json = |status: u16, body: String| Response {
        status,
        content_type: "application/json",
        body,
    };
    match (method, path) {
        ("GET", "/api/v1/health") | ("GET", "/api/v1/health/") => json(
            200,
            serde_json::to_string(&HealthResponse {
                status: "ok".into(),
                service: "todo-backend-rust".into(),
                db: "ok".into(),
            })
            .expect("health 序列化不可失败"),
        ),
        ("GET", "/api/v1/schema") | ("GET", "/api/v1/schema/") => Response {
            status: 200,
            content_type: "application/json",
            body: include_str!("../api/openapi.json").to_string(),
        },
        // 已知路径错误方法 → 405（对齐 DRF 行为）；其余 → 404
        (_, "/api/v1/health")
        | (_, "/api/v1/health/")
        | (_, "/api/v1/schema")
        | (_, "/api/v1/schema/") => json(405, r#"{"detail":"Method not allowed."}"#.into()),
        _ => json(404, r#"{"detail":"Not Found."}"#.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body_json(resp: &Response) -> serde_json::Value {
        serde_json::from_str(&resp.body).expect("响应体应为合法 JSON")
    }

    #[test]
    fn health_无尾斜杠与_django_形态都通() {
        for path in ["/api/v1/health", "/api/v1/health/"] {
            let resp = handle("GET", path);
            assert_eq!(resp.status, 200);
            assert_eq!(resp.content_type, "application/json");
            let v = body_json(&resp);
            assert_eq!(v["status"], "ok");
            assert_eq!(v["service"], "todo-backend-rust");
            assert_eq!(v["db"], "ok");
        }
    }

    #[test]
    fn schema_返回_openapi_文档() {
        for path in ["/api/v1/schema", "/api/v1/schema/"] {
            let resp = handle("GET", path);
            assert_eq!(resp.status, 200);
            let v = body_json(&resp);
            assert!(v["openapi"].is_string(), "缺 openapi 版本字段");
            assert!(
                v["paths"]["/api/v1/health/"].is_object(),
                "缺 health 路径声明"
            );
        }
    }

    #[test]
    fn 未知路径_404() {
        let resp = handle("GET", "/api/v1/nope");
        assert_eq!(resp.status, 404);
    }

    #[test]
    fn 已知路径错误方法_405() {
        let resp = handle("POST", "/api/v1/health");
        assert_eq!(resp.status, 405);
    }
}
