use std::error::Error;
use crate::setup::utils::{update_config_file, get_network_interfaces, ConfigValue};

pub async fn configure_network( wan_interface: String, lan_interfaces: Vec<String>, ) -> Result<(), Box<dyn Error>> {

    // Check that all given interfaces exist on the host
    let interfaces = get_network_interfaces();

    if !interfaces.contains(&wan_interface) {
        return Err(format!(
            "WAN interface '{}' does not exist on this host.",
            wan_interface
        ).into());
    }

    for lan in &lan_interfaces {
        if !interfaces.contains(lan) {
            return Err(format!(
                "LAN interface '{}' does not exist on this host.",
                lan
            ).into());
        }
    }

    if lan_interfaces.contains(&wan_interface) {
        return Err ("WAN interface cannot also be a LAN interface.".into() );
    }

    // Save interfaces to config
    update_config_file("network", "wan_interface", ConfigValue::String(wan_interface) )?;

    update_config_file("network", "lan_interfaces", ConfigValue::Array(lan_interfaces) )?;

    update_config_file("network", "configured", ConfigValue::Bool(true) )?;

    Ok(())
}