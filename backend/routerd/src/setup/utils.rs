use toml_edit::{DocumentMut, value};
use std::fs;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

pub fn update_config_file(section: &str, key: &str, new_value: &str) -> Result<(), Box<dyn std::error::Error>> {

    let content = fs::read_to_string("/etc/josdorOS/config.toml")?;
    let mut doc = content.parse::<DocumentMut>()?;

    let section_item = doc
        .get_mut(section)
        .ok_or(format!("Section [{}] missing", section))?;

    let section_table = section_item
        .as_table_mut()
        .ok_or(format!("[{}] is not a TOML table", section))?;

    if !section_table.contains_key(key) {
        return Err(format!("Key [{}] missing in section [{}]", key, section).into());
    }

    section_table[key] = value(new_value);

    fs::write("/etc/josdorOS/config.toml", doc.to_string())?;

    Ok(())
}

pub fn get_network_interfaces() -> Vec<String> {

    let interfaces = NetworkInterface::show().expect("Failed to get network interfaces");

    interfaces.into_iter().map(|iface| iface.name).filter(|name| {
        name != "lo" && !name.starts_with("docker") && !name.starts_with("veth") && !name.starts_with("virbr") && !name.starts_with("br-") && !name.starts_with("tun") // Filter
    }).collect()

}