use std::error::Error;
use crate::firewall::config::apply_nftables;
use crate::setup::config::{configure_lan_bridge, enable_ip_forwarding};
use crate::setup::utils::{update_config_file, get_network_interfaces, ConfigValue, install_dhcp_server, mask_to_cidr};

pub async fn configure_network( wan_interface: String, lan_interfaces: Vec<String>, dhcp_ip_range_start: String, dhcp_ip_range_end: String, forwarding_ip : String, dhcp_lease: String, mask: String ) -> Result<(), Box<dyn Error>> {

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
    update_config_file("dhcp", "forwarding_ip", ConfigValue::String(forwarding_ip.clone()))?;
    update_config_file("dhcp", "lease", ConfigValue::String(dhcp_lease.clone()))?;
    update_config_file("dhcp", "mask", ConfigValue::String(mask.clone()))?;

    let cidr = mask_to_cidr(&mask);
    let ip_with_cidr = format!("{}/{}", forwarding_ip, cidr);

    println!("{}", ip_with_cidr);
    apply_basic_routing(&lan_interfaces, ip_with_cidr)?;
    install_dhcp_server(dhcp_ip_range_start.as_str(), dhcp_ip_range_end.as_str(), forwarding_ip.as_str(), dhcp_lease.as_str())?;

    Ok(())
}

pub fn apply_basic_routing (lan_interfaces: &[String], ip_with_cidr: String) -> Result<(), Box<dyn Error>> {

    enable_ip_forwarding()?;
    configure_lan_bridge(Vec::from(lan_interfaces), &ip_with_cidr).expect("Cannot create Lan Bridge");
    apply_nftables()?;

    Ok(())
}