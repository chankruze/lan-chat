mod config;
mod networking;
mod peer;
mod utils;

use networking::{discover, start_mdns_service_with_re_advertise};
use utils::logger;

use peer::PeerMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Initialize logger once
  logger::init_logger();

  let peer_id = utils::generate_peer_id();
  let peer_name = utils::generate_peer_name();
  let instance_name = utils::generate_instance_name(&peer_name, &peer_id);

  log::info!(
    "\n\
        🚀 Starting LAN Chat\n\
        ├─ ID      : {peer_id}\n\
        ├─ Name    : {peer_name}\n\
        ├─ Instance: {instance_name}\n\
        └─ Port    : {}",
    config::SERVICE_PORT
  );

  let peers: PeerMap = Arc::new(tokio::sync::RwLock::new(Default::default()));

  let (advertise_tx, advertise_rx) = tokio::sync::mpsc::unbounded_channel();

  let advertise_task = tokio::spawn({
    let peer_id = peer_id.clone();

    async move {
      start_mdns_service_with_re_advertise(&peer_id, &peer_name, &instance_name, advertise_rx).await
    }
  });

  let discover_task = tokio::spawn({
    let peer_id = peer_id.clone();
    let peers = peers.clone();
    async move {
      if let Err(e) = discover(peer_id, peers, advertise_tx).await {
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
