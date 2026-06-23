use std::process::Command;
use std::path::Path;
use std::fs;
use crate::setup::network::apply_basic_routing;
use crate::setup::utils::load_config;

const CONFIG_DIR: &str = "/etc/josdorOS";
const CONFIG_PATH: &str = "/etc/josdorOS/config.toml";
const DEFAULT_CONFIG: &str = include_str!("default_config.toml");

pub fn create_config() {
    println!("Creating configuration...");

    fs::create_dir_all(CONFIG_DIR)
        .expect("Failed to create configuration directory");

    fs::write(CONFIG_PATH, DEFAULT_CONFIG)
        .expect("Failed to write configuration file");

    if !Path::new(CONFIG_PATH).exists() {
        panic!("Configuration file creation failed");
    }

    println!("Configuration created at {}", CONFIG_PATH);
}

pub fn enable_ip_forwarding() -> Result<(), Box<dyn std::error::Error>> {
    fs::write("/proc/sys/net/ipv4/ip_forward", "1")?;
    Ok(())
}

pub fn configure_lan_ip(lan_interface: &str) -> Result<(), Box<dyn std::error::Error>> {

    Command::new("ip").args(["addr", "flush", "dev", lan_interface]).status()?;
    Command::new("ip").args(["addr", "add", "10.10.0.1/24", "dev", lan_interface]).status()?;
    Command::new("ip").args(["link", "set", lan_interface, "up"]).status()?;

    Ok(())
}

pub fn load_from_existing_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("Configuration file already exists at /etc/josdorOS/config.toml. Booting with existing configuration... ");

    let config = load_config();

    apply_basic_routing(&*config.network.lan_interfaces).expect("Failed to apply basic routing, check the config file.");
    Ok(())
}