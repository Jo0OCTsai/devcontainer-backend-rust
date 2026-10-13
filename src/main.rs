//! 入口层（对齐 backend 的 config/ 角色）：只做装配与监听，业务逻辑在 app 模块。

mod app;

fn main() {
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:8002".into());
    let server = tiny_http::Server::http(&addr).unwrap_or_else(|e| panic!("监听 {addr} 失败: {e}"));
    eprintln!("todo-backend-rust listening on {addr}");
    for request in server.incoming_requests() {
        let resp = app::handle(request.method().as_str(), request.url());
        let header =
            tiny_http::Header::from_bytes(&b"Content-Type"[..], resp.content_type.as_bytes())
                .expect("Content-Type 头构造不可失败");
        let response = tiny_http::Response::from_string(resp.body)
            .with_status_code(resp.status)
            .with_header(header);
        request.respond(response).expect("写响应失败");
    }
}
