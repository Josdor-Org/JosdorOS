use toml_edit::{value, Array, DocumentMut, Item};
use std::fs;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NConfig {
    pub network: NetworkConfig,
    pub dhcp: DHCPConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub configured: bool,
    pub wan_interface: String,
    pub lan_interfaces: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DHCPConfig {
    pub enabled: bool,
    pub ip_range_start : String,
    pub ip_range_end : String,
    pub forwarding_ip: String,
    pub lease : String
}

pub enum ConfigValue {
    String(String),
    Bool(bool),
    Array(Vec<String>),
}

#[derive(Debug, Serialize)]
pub struct InterfaceInfo {
    pub name: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub mac: String,
    pub speed: Option<u32>,
    pub state: String,
}

#[derive(Debug, Deserialize)]
struct LshwInterface {
    logicalname: Option<String>,
    vendor: Option<String>,
    product: Option<String>,
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

fn get_lshw_interfaces() -> Vec<LshwInterface> {
    let output = Command::new("lshw").args(["-class", "network", "-json"]).output().expect("Failed to run lshw");

    let json = String::from_utf8_lossy(&output.stdout);

    serde_json::from_str(&json)
        .unwrap_or_else(|_| Vec::new())
}

pub fn get_network_interfaces_all_infos() -> Vec<InterfaceInfo> {

    let interfaces = get_network_interfaces();
    let lshw_interfaces = get_lshw_interfaces();

    let mut result = Vec::new();

    for interface in interfaces {

        let speed = fs::read_to_string(
            format!("/sys/class/net/{}/speed", interface)
        )
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok());

        let state = fs::read_to_string(
            format!("/sys/class/net/{}/operstate", interface)
        )
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string();

        let mac = fs::read_to_string(
            format!("/sys/class/net/{}/address", interface)
        )
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string();

        let hw = lshw_interfaces
            .iter()
            .find(|i| i.logicalname.as_deref() == Some(interface.as_str()));

        result.push(InterfaceInfo {
            name: interface,
            vendor: hw.and_then(|i| i.vendor.clone()),
            model: hw.and_then(|i| i.product.clone()),
            mac,
            speed,
            state,
        });
    }

    result
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

fn apt_update()-> Result<(), Box<dyn std::error::Error>> {

    Command::new("sudo").arg("apt").arg("update").status()?;
    Ok(())
}

pub fn install_dhcp_server(lan_interface: &str, ip_range_start: &str, ip_range_end: &str, forwarding_ip : &str, lease: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = format!(
        r#"interface={}
bind-interfaces
dhcp-range={},{},{}
dhcp-option=3,{}
dhcp-option=6,1.1.1.1,8.8.8.8
"#,
        lan_interface,
        ip_range_start,
        ip_range_end,
        lease,
        forwarding_ip,
    );

    apt_update()?;
    Command::new("sudo").args(&["apt", "install", "dnsmasq", "-y"]).status()?;

    fs::write("/etc/dnsmasq.d/josdor.conf", config)?;

    Command::new("systemctl").args(["restart", "dnsmasq"]).status()?;
    Command::new("systemctl").args(["enable", "dnsmasq"]).status()?;

    Ok(())
}

pub fn install_lshw() -> Result<(), Box<dyn std::error::Error>> {
    
    apt_update()?;
    let _out = Command::new("sudo").args(&["apt", "install", "lshw", "-y"]).status()?;
    
    Ok(())
}