pub mod config;
pub mod networking;
pub mod peer;
pub mod utils;

use crate::utils::{generate_instance_name, generate_peer_id, generate_peer_name, logger};
use networking::{discover, start_mdns_service_with_re_advertise};
use peer::{PeerEvent, PeerMap, PeerNotifier};
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct LanChatRuntime {
  pub peer_map: PeerMap,
  pub peer_event_rx: UnboundedReceiver<PeerEvent>,
}

pub async fn start_lan_chat() -> anyhow::Result<LanChatRuntime> {
  logger::init_logger();

  let peer_id = generate_peer_id();
  let peer_name = generate_peer_name();
  let instance_name = generate_instance_name(&peer_name, &peer_id);

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
  let (peer_event_tx, peer_event_rx) = tokio::sync::mpsc::unbounded_channel::<PeerEvent>();
  let notifier = PeerNotifier::new(advertise_tx.clone(), peer_event_tx.clone());

  tokio::spawn({
    let peer_id = peer_id.clone();
    let peer_name = peer_name.clone();
    let instance_name = instance_name.clone();
    async move {
      start_mdns_service_with_re_advertise(&peer_id, &peer_name, &instance_name, advertise_rx)
        .await;
    }
  });

  tokio::spawn({
    let peer_id = peer_id.clone();
    let peers = peers.clone();
    let notifier = notifier.clone();
    async move {
      if let Err(e) = discover(peer_id, peers, notifier).await {
        eprintln!("❌ Discovery error: {e:?}");
      }
    }
  });

  Ok(LanChatRuntime {
    peer_map: peers,
    peer_event_rx,
  })
}
