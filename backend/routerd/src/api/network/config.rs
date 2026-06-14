use axum::Json;
use crate::setup::utils::load_config;
use crate::setup::utils::NetworkConfig;

pub async fn get_network_config() -> Json<NetworkConfig> {

    let config = load_config();

    Json(config.network)
}