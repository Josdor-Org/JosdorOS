use std::process::Command;

fn create_config() {
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

pub fn init() {
    
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