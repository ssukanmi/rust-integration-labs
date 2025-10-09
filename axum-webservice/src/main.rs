#![allow(unused, clippy::unwrap_used)] // For bootstraping

use std::net::SocketAddr;

use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let route_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, route_all).await.unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", routing::get(handler_hello))
        .route("/hello2/{name}", routing::get(handler_hello2))
}

fn routes_static() -> Router {
    // Router::new().fallback_service("/", routing::get_service(ServeDir::new("./")))
    Router::new().fallback_service(ServeDir::new("./"))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// `/hello?name=Sukanmi`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->>{:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

// `/hello2/name`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->>{:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
