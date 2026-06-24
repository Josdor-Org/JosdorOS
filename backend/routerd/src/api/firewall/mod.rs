use axum::Json;
use serde::Deserialize;
use firewall::config::FirewallRule;
use crate::firewall;
use crate::firewall::config::{add_firewall_rule, apply_nftables, delete_firewall_rule, FirewallConfig};
use crate::setup::utils::load_config;


#[derive(Debug, Deserialize)]
pub struct DeleteFirewallRule {
    pub name: String,
}


pub async fn handle_create_firewall_rule(Json(payload): Json<FirewallRule>) -> String {

    match add_firewall_rule(&*payload.name, payload.enabled, &payload.source, &payload.destination, &payload.protocol, payload.port, &payload.action ).await {
        Ok(_) => "Created firewall rule".to_string(),
        Err(e) => format!("Failed to create firewall rule: {}", e)
    };

    match apply_nftables() {
        Ok(_) => "Firewall rule applied".to_string(),
        Err(e) => format!("Failed to apply firewall rule: {}", e)
    }
}

pub async fn handle_show_firewall_rules() -> Json<FirewallConfig> {

    let config = load_config();

    Json(config.firewall)
}

pub async fn handle_delete_firewall_rule(Json(payload): Json<DeleteFirewallRule>) -> String {

    match delete_firewall_rule(&payload.name).await {
        Ok(_) => "Deleted firewall rule".to_string(),
        Err(e) => format!("Failed to delete firewall rule: {}", e)
    }
}