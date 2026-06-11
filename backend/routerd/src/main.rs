mod api;

use api::startAPI;

#[tokio::main]
async fn main() {
    println!("Welcome to JosdorOS Router Daemon!");
    println!("Starting API server...");
    api::startAPI().await;
}
