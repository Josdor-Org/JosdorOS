mod api;
mod setup;
mod firewall;
mod system;

#[tokio::main]
async fn main() {
    println!("Welcome to JosdorOS Router Daemon!");
    setup::boot();

    println!("Starting API server...");

    api::start_api().await;

}