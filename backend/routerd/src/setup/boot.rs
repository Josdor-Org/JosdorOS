use std::process::Command;
use crate::setup::config::create_config;
use nix::unistd::Uid;

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
        println!("Configuration file already exists at /etc/josdorOS/config.toml. Booting with existing configuration... ");
    }
}