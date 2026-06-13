use axum::{Json};
use serde::Deserialize;

use crate::setup::network::configure_network;


#[derive(Deserialize)]
pub struct NetworkSetupRequest {
    pub hostname: String,
    pub wan_interface: String,
    pub lan_interfaces: Vec<String>,
}

pub async fn setup_network(Json(payload): Json<NetworkSetupRequest>) -> String {

    match configure_network(payload.hostname, payload.wan_interface, payload.lan_interfaces).await{
        Ok(_) => "Network setup completed successfully.".to_string(),
        Err(e) => format!("Network setup failed: {}", e),
    }
} 