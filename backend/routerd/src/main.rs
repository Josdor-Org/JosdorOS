mod api;
mod setup;

use api::startAPI;

#[tokio::main]
async fn main() {
    println!("Welcome to JosdorOS Router Daemon!");
    setup::init();

    println!("Starting API server...");
    api::startAPI().await;
}
