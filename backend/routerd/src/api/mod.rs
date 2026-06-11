mod health;

use axum::{Router, routing::post};
use tokio::net::TcpListener;


pub async fn startAPI(){
    let api: Router = Router::new()
        .route("/api/health", post(health::health));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, api).await.unwrap();
}