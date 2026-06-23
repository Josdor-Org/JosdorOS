use std::error::Error;
use std::process::Command;
use serde::{Deserialize, Serialize};
use setup::utils::apt_update;

use crate::setup;

#[derive(Debug, Deserialize, Serialize)]
pub struct FirewallConfig {
    pub enabled: bool,
    pub rules: Vec<FirewallRule>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FirewallRule {
    pub enabled: bool,
    pub name: String,
    pub source: String,
    pub destination: String,
    pub protocol: String,
    pub port: Option<u16>,
    pub action: String,
}

pub fn install_nftables() -> Result<(), Box<dyn Error>> {

    apt_update().expect("Cannot run command : apt update");

    let output = Command::new("apt").args(&["install", "nftables"]).output()?;

    if !output.status.success() {
        Err(format!("Apt installation failed: {:?}", output.stderr).into())
    }else {
        Ok(())
    }
}

