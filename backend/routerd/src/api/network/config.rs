use axum::Json;
use crate::setup::utils::{get_network_interfaces_all_infos, load_config, InterfaceInfo};
use crate::setup::utils::NetworkConfig;

pub async fn get_network_config() -> Json<NetworkConfig> {

    let config = load_config();

    Json(config.network)
}

pub async fn get_network_interfaces_infos() -> Json<Vec<InterfaceInfo>> {

    let config = get_network_interfaces_all_infos();

    Json(config)
}