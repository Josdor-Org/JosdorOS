use toml_edit::{value, Array, DocumentMut, Item};
use std::fs;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::process::Command;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct NConfig {
    pub network: NetworkConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub configured: bool,
    pub wan_interface: String,
    pub lan_interfaces: Vec<String>,
}

pub enum ConfigValue {
    String(String),
    Bool(bool),
    Array(Vec<String>),
}

pub fn update_config_file( section: &str, key: &str, new_value: ConfigValue ) -> Result<(), Box<dyn std::error::Error>> {

    let content = fs::read_to_string("/etc/josdorOS/config.toml")?;
    let mut doc = content.parse::<DocumentMut>()?;

    let section_item = doc
        .get_mut(section)
        .ok_or(format!("Section [{}] missing", section))?;

    let section_table = section_item
        .as_table_mut()
        .ok_or(format!("[{}] is not a TOML table", section))?;

    if !section_table.contains_key(key) {
        return Err(
            format!(
                "Key [{}] missing in section [{}]",
                key,
                section
            )
                .into(),
        );
    }

    let item: Item = match new_value {
        ConfigValue::String(v) => value(v),

        ConfigValue::Bool(v) => value(v),

        ConfigValue::Array(values) => {
            let mut arr = Array::default();

            for v in values {
                arr.push(v);
            }

            value(arr)
        }
    };

    section_table[key] = item;

    fs::write(
        "/etc/josdorOS/config.toml",
        doc.to_string(),
    )?;

    Ok(())
}

pub fn get_network_interfaces() -> Vec<String> {

    let interfaces = NetworkInterface::show().expect("Failed to get network interfaces");

    interfaces.into_iter().map(|iface| iface.name).filter(|name| {
        name != "lo" && !name.starts_with("docker") && !name.starts_with("veth") && !name.starts_with("virbr") && !name.starts_with("br-") && !name.starts_with("tun") // Filter
    }).collect()

}

pub async fn set_hostname(hostname: &str) -> Result<(), Box<dyn std::error::Error>> {

    let out = Command::new("hostnamectl").arg("set-hostname").arg(hostname).status();

    if out.is_err() {
        return Err(out.unwrap_err().into());
    }

    update_config_file("system", "hostname", ConfigValue::String(hostname.into()))?;

    Ok(())
}

pub fn load_config() -> NConfig {

    let content = fs::read_to_string("/etc/josdorOS/config.toml").expect("Failed to read config file");

    toml::from_str(&content).expect("Failed to parse config file")
}