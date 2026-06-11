use serde::Deserialize;

#[derive(Deserialize)]
pub struct NetworkSetupRequest {
    pub wan_interface: String,
    pub lan_interface: Vec<String>,
}