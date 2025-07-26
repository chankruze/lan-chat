use lan_chat::{
  networking::{discover, start_mdns_service_with_re_advertise},
  peer::{PeerEvent, PeerIdentity, PeerMap, PeerNotifier},
  utils::logger,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Initialize logger once
  logger::init_logger();

  let identity = Arc::new(RwLock::new(PeerIdentity::load_or_generate()));

  let peers: PeerMap = Arc::new(tokio::sync::RwLock::new(Default::default()));

  let (advertise_tx, advertise_rx) = tokio::sync::mpsc::unbounded_channel();
  let (peer_event_tx, _peer_event_rx) = tokio::sync::mpsc::unbounded_channel::<PeerEvent>();

  let notifier = PeerNotifier::new(advertise_tx.clone(), peer_event_tx.clone());

  let advertise_task = tokio::spawn({
    let identity = identity.clone();

    async move { start_mdns_service_with_re_advertise(identity, advertise_rx).await }
  });

  let discover_task = tokio::spawn({
    let peer_id = identity.read().await.peer_id.clone();
    let peers = peers.clone();
    let notifier = notifier.clone();

    async move {
      if let Err(e) = discover(peer_id, peers, notifier).await {
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
