use std::process::Command;

pub fn create_config() {
    println!("Creating configuration...");
    Command::new("mkdir")
        .arg("-p")
        .arg("/etc/josdorOS")
        .output()
        .expect("Failed to create configuration directory");

    Command::new("cp")
        .arg("src/setup/default_config.toml")
        .arg("/etc/josdorOS/config.toml")
        .output()
        .expect("Failed to copy configuration file");

    if !Command::new("ls")
        .arg("/etc/josdorOS/config.toml")
        .output()
        .expect("Failed to verify configuration file")
        .status
        .success()
    {
        panic!("Configuration file creation failed");
    }

    println!("Configuration created at /etc/josdorOS");

}