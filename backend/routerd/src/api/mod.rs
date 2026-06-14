mod health;
mod monitoring;
mod setup;
mod network;

use axum::{Router, routing::post, routing::get};
use tokio::net::TcpListener;


pub async fn start_api(){
    let api: Router<()> = Router::new()
        .route("/api/health", get(health::health))
        .route("/api/hostname", get(monitoring::hostname))
        .route("/api/setup", post(setup::setup_network))
        .route("/api/network/config", get(network::config::get_network_config));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, api).await.unwrap();
}