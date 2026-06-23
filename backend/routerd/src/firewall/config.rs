use std::error::Error;
use std::process::Command;
use serde::{Deserialize, Serialize};
use toml_edit::{value, DocumentMut, Table};
use setup::utils::apt_update;
use std::fs;
use crate::setup;
use crate::setup::utils::load_config;

#[derive(Debug, Deserialize, Serialize)]
pub struct FirewallConfig {
    pub enabled: bool,

    #[serde(default)]
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

pub async fn add_firewall_rule(name: &str, enabled: bool, source: &str, destination: &str, protocol: &str, port: Option<u16>, action: &str) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string("/etc/josdorOS/config.toml")?;
    let mut doc = content.parse::<DocumentMut>()?;

    let mut rule = Table::new();
    rule["name"] = value(name);
    rule["enabled"] = value(enabled);
    rule["source"] = value(source);
    rule["destination"] = value(destination);
    rule["protocol"] = value(protocol);
    if let Some(port) = port {
        rule["port"] = value(port as i64);
    }
    rule["action"] = value(action);

    doc["firewall"]["rules"].or_insert(toml_edit::Item::ArrayOfTables(Default::default())).as_array_of_tables_mut().unwrap().push(rule);
    fs::write("/etc/josdorOS/config.toml", doc.to_string())?;

    Ok(())
}

pub fn apply_firewall() -> Result<(), Box<dyn Error>> {
    let config = load_config();

    let mut nft = String::new();

    nft.push_str(
        r#"
table inet filter {
    chain input {
        type filter hook input priority 0;
        policy drop;

        ct state established,related accept
        iif lo accept
"#,
    );

    for rule in config.firewall.rules {
        if !rule.enabled {
            continue;
        }

        let action = match rule.action.as_str() {
            "accept" => "accept",
            "drop" => "drop",
            _ => continue,
        };

        match rule.protocol.as_str() {
            "tcp" => {
                if let Some(port) = rule.port {
                    nft.push_str(
                        format!("        tcp dport {} {}\n", port, action).as_str(),
                    );
                }
            }
            "udp" => {
                if let Some(port) = rule.port {
                    nft.push_str(
                        format!("        udp dport {} {}\n", port, action).as_str(),
                    );
                }
            }
            _ => {}
        }
    }

    nft.push_str(
        r#"
    }
}
"#,
    );

    fs::write("/etc/nftables.conf", nft)?;

    Command::new("nft")
        .args(["-f", "/etc/nftables.conf"])
        .status()?;

    Ok(())
}

pub async fn delete_firewall_rule(name: &str) -> Result<(), Box<dyn Error>> {
    
    let content = fs::read_to_string("/etc/josdorOS/config.toml")?;
    let mut doc = content.parse::<DocumentMut>()?;

    let rules = doc["firewall"]["rules"].as_array_of_tables_mut().ok_or("No firewall rules found")?;

    let pos = rules.iter().position(|rule| rule["name"].as_str() == Some(name));

    if let Some(pos) = pos {
        rules.remove(pos);
    }

    fs::write("/etc/josdorOS/config.toml", doc.to_string())?;

    Ok(())
}