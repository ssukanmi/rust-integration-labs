#![allow(clippy::unwrap_used)]

use axum::{Router, routing};
use tokio::net::TcpListener;

use crate::controller::{get_info_handler, login_handler};

mod controller;
mod model;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", routing::post(login_handler))
        .route("/info", routing::get(get_info_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening......s");

    axum::serve(listener, app).await.unwrap();
}
