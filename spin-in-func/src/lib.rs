use serde_json::json;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
async fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.get("/", health);
    router.get("/api/spin-in-func", handle_spin_in_func);
    router.handle(req)
}

fn health(_req: Request, _param: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::new(200, format!("Healthy")))
}

fn handle_spin_in_func(req: Request, _param: Params) -> anyhow::Result<impl IntoResponse> {
    let payload = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!("url {} / payload {}", req.uri(), payload.to_string());
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(payload.to_string())
        .build())
}
