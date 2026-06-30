use axum::{
    Json, Router,
    body::Body,
    http::{Response, StatusCode, Uri, header},
    routing::get,
};
use rust_embed::RustEmbed;
use serde_json::{Value, json};
use std::net::SocketAddr;

#[derive(RustEmbed)]
#[folder = "../build/"]
struct Assets;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let api_routes = Router::new().route("/health", get(health));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(static_handler);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn static_handler(uri: Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');
    serve_asset(path)
}

fn serve_asset(path: &str) -> Response<Body> {
    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => serve_asset("index.html"),
    }
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
