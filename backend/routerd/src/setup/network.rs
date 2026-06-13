use std::{error::Error, fs};
use toml_edit::{DocumentMut, value, Array};
use crate::setup::utils::update_config_file;

pub async fn configure_network(hostname: String, wan_interface: String, lan_interfaces: Vec<String>) -> Result<(), Box<dyn Error>> {

    let mut lan_array = Array::default();

    for interface in lan_interfaces {
        lan_array.push(interface);
    }

    update_config_file("system", "hostname", &hostname)?;
    update_config_file("network", "wan_interface", &wan_interface)?;
    update_config_file("network", "lan_interfaces", &lan_array.to_string())?;

    Ok(())
}