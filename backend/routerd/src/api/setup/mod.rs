use axum::{Json};
use serde::Deserialize;

use crate::setup::network::configure_network;
use crate::setup::utils::set_hostname;


#[derive(Deserialize)]
pub struct NetworkSetupRequest {
    pub hostname: String,
    pub wan_interface: String,
    pub lan_interfaces: Vec<String>,
}

pub async fn setup_network(Json(payload): Json<NetworkSetupRequest>) -> String {

    match configure_network(payload.wan_interface, payload.lan_interfaces).await {
        Ok(_) => "Network setup completed successfully.".to_string(),
        Err(e) => format!("Network setup failed: {}", e),
    };
    // Now change the hostname by the one provided in the request

    match set_hostname(payload.hostname.as_str()).await {
        Ok(_) => "Network setup completed successfully.".to_string(),
        Err(e) => format!("Network setup failed: {}", e),
    }

} 