mod health;
mod monitoring;

use axum::{Router, routing::post, routing::get};
use tokio::net::TcpListener;


pub async fn startAPI(){
    let api: Router = Router::new()
        .route("/api/health", post(health::health))
        .route("/api/hostname", post(monitoring::hostname));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, api).await.unwrap();
}