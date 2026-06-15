use std::error::Error;
use crate::nftables::config::configure_nat;
use crate::setup::config::{configure_lan_ip, enable_ip_forwarding};
use crate::setup::utils::{update_config_file, get_network_interfaces, ConfigValue, install_dns_server};

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
    update_config_file("network", "wan_interface", ConfigValue::String(wan_interface.clone()))?;
    update_config_file("network", "lan_interfaces", ConfigValue::Array(lan_interfaces.clone()))?;
    update_config_file("network", "configured", ConfigValue::Bool(true) )?;

    apply_basic_routing(&wan_interface, &lan_interfaces)?;
    install_dns_server(&lan_interfaces[0])?;

    Ok(())
}

pub fn apply_basic_routing (wan_interface: &str, lan_interfaces: &[String]) -> Result<(), Box<dyn Error>> {
    let lan = lan_interfaces.first().ok_or("No LAN interface configured")?;

    enable_ip_forwarding()?;
    configure_lan_ip(lan)?;
    configure_nat(wan_interface)?;

    Ok(())
}