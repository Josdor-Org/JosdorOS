use std::process::Command;
use crate::setup::config::{create_config, load_from_existing_config};
use nix::unistd::Uid;
use setup::utils::install_lshw;
use crate::firewall::config::install_nftables;
use crate::setup;

fn check_root() {
    if !Uid::effective().is_root() {
        panic!("This program must be run as root.");
    } else {
        println!("Root privileges confirmed.");
    }
}

pub fn boot() {
    println!("Checking for root privileges...");
    check_root();
    println!("Booting routerd...");

    let output = Command::new("ls")
        .arg("/etc/josdorOS/config.toml")
        .output()
        .expect("Failed to check configuration file");

    if !output.status.success() {
        create_config();
    } else {
        load_from_existing_config().expect("Failed to apply basic routing, check the config file.");
    }
    
    println!("Installing mandatory firmware versions...");
    install_lshw().expect("Failed to install lshw");
    install_nftables().expect("Failed to install nftables");

    println!("Boot successfully!");
}