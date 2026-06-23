use std::error::Error;
use crate::setup::config::{configure_lan_ip, enable_ip_forwarding};
use crate::setup::utils::{update_config_file, get_network_interfaces, ConfigValue, install_dhcp_server};

pub async fn configure_network( wan_interface: String, lan_interfaces: Vec<String>, dhcp_ip_range_start: String, dhcp_ip_range_end: String, dhcp_forwarding_ip : String, dhcp_lease: String ) -> Result<(), Box<dyn Error>> {

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

    // Save interfaces to config file
    update_config_file("network", "wan_interface", ConfigValue::String(wan_interface.clone()))?;
    update_config_file("network", "lan_interfaces", ConfigValue::Array(lan_interfaces.clone()))?;
    update_config_file("network", "configured", ConfigValue::Bool(true) )?;

    // Save DHCP config to config file
    update_config_file("dhcp", "enabled", ConfigValue::Bool(true))?;
    update_config_file("dhcp", "ip_range_start", ConfigValue::String(dhcp_ip_range_start.clone()))?;
    update_config_file("dhcp", "ip_range_end", ConfigValue::String(dhcp_ip_range_end.clone()))?;
    update_config_file("dhcp", "forwarding_ip", ConfigValue::String(dhcp_forwarding_ip.clone()))?;
    update_config_file("dhcp", "lease", ConfigValue::String(dhcp_lease.clone()))?;

    apply_basic_routing(&lan_interfaces)?;
    install_dhcp_server(&lan_interfaces[0], dhcp_ip_range_start.as_str(), dhcp_ip_range_end.as_str(), dhcp_forwarding_ip.as_str(), dhcp_lease.as_str())?;

    Ok(())
}

pub fn apply_basic_routing (lan_interfaces: &[String]) -> Result<(), Box<dyn Error>> {
    let lan = lan_interfaces.first().ok_or("No LAN interface configured")?;

    enable_ip_forwarding()?;
    configure_lan_ip(lan)?;

    Ok(())
}