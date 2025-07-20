mod advertise_async;
mod discovery;
mod peer;

use peer::PeerMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let peer_id = uuid::Uuid::new_v4().to_string();
    let peer_name = whoami::username();
    println!("🚀 Starting LAN Chat as {peer_name} ({peer_id})");

    let peers: PeerMap = Arc::new(tokio::sync::RwLock::new(Default::default()));

    let advertise_task = tokio::spawn({
        let peer_id = peer_id.clone(); // Clone peer_id here so the move block owns it
        async move {
            advertise_async::start_mdns_service(
                "_lan-chat._tcp", // Custom service type (must match discovery)
                "RustChat",       // Instance name (e.g., hostname-like)
                8080,             // Port number
                &["peer_id", &peer_id],
            )
            .await
        }
    });

    let discover_task = tokio::spawn({
        let peer_id = peer_id.clone();
        let peers = peers.clone();
        async move {
            if let Err(e) = discovery::discover(peer_id, peers).await {
                eprintln!("❌ Discovery error: {e:?}");
            }
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("🛑 Shutting down gracefully (Ctrl+C)");
        }
        _ = advertise_task => {
            println!("📡 mDNS advertisement ended");
        }
        _ = discover_task => {
            println!("🔍 mDNS discovery ended");
        }
    }

    Ok(())
}
