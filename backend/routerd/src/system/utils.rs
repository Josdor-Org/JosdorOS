use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::setup::utils::load_config;

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemStatus {
    routerd: bool,
    configured: bool,
    dhcp: bool,
    firewall: bool,
    ip_forwarding: bool,
    internet: bool,
}

fn is_routerd_running() -> bool {

    Command::new("systemctl").args(["is-active", "--quiet", "routerd"]).status().map(|s| s.success()).unwrap_or(false)
}

fn internet_access() -> bool {
    Command::new("ping").args(["-c", "1", "-W", "2", "google.com"]).status().map(|s| s.success()).unwrap_or(false)
}

fn is_ip_forwarding_enabled() -> bool {
    std::fs::read_to_string("/proc/sys/net/ipv4/ip_forward").map(|v| v.trim() == "1").unwrap_or(false)
}

pub fn system_status() -> SystemStatus {
    let config = load_config();

    SystemStatus {
        routerd: is_routerd_running(),
        configured: config.network.configured,
        dhcp: config.dhcp.enabled,
        firewall: config.firewall.enabled,
        ip_forwarding: is_ip_forwarding_enabled(),
        internet: internet_access(),
    }
}