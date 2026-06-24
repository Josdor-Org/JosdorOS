use axum::Json;
use crate::system::utils::{SystemStatus, system_status};

pub async fn hostname() -> String {
    match hostname::get() {
        Ok(name) => name.into_string().unwrap_or_else(|_| "Unknown".to_string()),
        Err(_) => "Unknown".to_string(),
    }
}

pub async fn status() -> Json<SystemStatus> {
    
    let config = system_status();
    Json(config)
}