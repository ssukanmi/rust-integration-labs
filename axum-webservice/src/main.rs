#![allow(unused)] // For bootstraping

use std::net::SocketAddr;

use axum::{response::Html, routing, Router};

#[tokio::main]
async fn main() {
    let route_hello = Router::new().route(
        "/hello",
        routing::get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    let addr = SocketAddr::from(([127,0,0,1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("->> LISTENING on {addr}\n");
    axum::serve(listener, route_hello).await.unwrap();
}
