mod discovery;
mod terminal;
mod ws_client;
mod ws_server;

use anyhow::Result;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let peer_id = Uuid::new_v4().to_string();
    let peer_name = whoami::username();
    println!("Starting LAN Chat as {peer_name} ({peer_id})");

    // Start WebSocket server, mDNS advertise, discover, connect...
    // Start terminal interface
    Ok(())
}
