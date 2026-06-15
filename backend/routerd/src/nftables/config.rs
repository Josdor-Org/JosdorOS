pub fn configure_nat(wan_interface: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = format!(r#"#!/usr/sbin/nft -f

flush ruleset

table inet filter {{
    chain input {{
        type filter hook input priority 0;
        policy accept;
    }}

    chain forward {{
        type filter hook forward priority 0;
        policy accept;
    }}

    chain output {{
        type filter hook output priority 0;
        policy accept;
    }}
}}

table ip nat {{
    chain postrouting {{
        type nat hook postrouting priority 100;
        oifname "{}" masquerade
    }}
}}
"#, wan_interface);

    std::fs::write("/etc/nftables.conf", config)?;

    std::process::Command::new("nft")
        .args(["-f", "/etc/nftables.conf"])
        .status()?;

    Ok(())
}