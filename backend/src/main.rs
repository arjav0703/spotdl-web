use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "dist".to_string());

    let api_routes = Router::new()
        .route("/health", get(health));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(
            ServeDir::new(&static_dir)
                .not_found_service(ServeFile::new(format!("{}/index.html", static_dir))),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
