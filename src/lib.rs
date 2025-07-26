pub mod config;
pub mod networking;
pub mod peer;
pub mod utils;

use networking::{discover, start_mdns_service_with_re_advertise};
use peer::{PeerEvent, PeerIdentity, PeerMap, PeerNotifier};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc::UnboundedReceiver};
use utils::logger;

pub struct LanChatRuntime {
  pub peer_map: PeerMap,
  pub peer_event_rx: UnboundedReceiver<PeerEvent>,
  pub identity: Arc<RwLock<PeerIdentity>>,
  pub notifier: PeerNotifier,
}

pub async fn start_lan_chat() -> anyhow::Result<LanChatRuntime> {
  logger::init_logger();

  let identity = Arc::new(RwLock::new(PeerIdentity::load_or_generate()));
  let peers: PeerMap = Arc::new(tokio::sync::RwLock::new(Default::default()));
  let (advertise_tx, advertise_rx) = tokio::sync::mpsc::unbounded_channel();
  let (peer_event_tx, peer_event_rx) = tokio::sync::mpsc::unbounded_channel::<PeerEvent>();
  let notifier = PeerNotifier::new(advertise_tx.clone(), peer_event_tx.clone());

  tokio::spawn({
    let identity = identity.clone();

    async move {
      start_mdns_service_with_re_advertise(identity, advertise_rx).await;
    }
  });

  tokio::spawn({
    let peer_id = identity.read().await.peer_id.clone();
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
    identity,
    notifier,
  })
}
