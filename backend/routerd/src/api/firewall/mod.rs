use axum::Json;
use firewall::config::FirewallRule;
use crate::firewall;
use crate::firewall::config::{add_firewall_rule, apply_firewall};

pub async fn handle_create_firewall_rule(Json(payload): Json<FirewallRule>) -> String {

    match add_firewall_rule(&*payload.name, payload.enabled, &payload.source, &payload.destination, &payload.protocol, payload.port, &payload.action ).await {
        Ok(_) => "Created firewall rule".to_string(),
        Err(e) => format!("Failed to create firewall rule: {}", e)
    };
    
    match apply_firewall() { 
        Ok(_) => "Firewall rule applied".to_string(),
        Err(e) => format!("Failed to apply firewall rule: {}", e)
    }
}