use super::{PeerEvent, PeerInfo, PeerMetadata};
use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
  time::{Duration, Instant},
};
use tokio::sync::{RwLock, mpsc};

#[derive(Debug, Clone)]
pub struct ManagedPeer {
  pub info: PeerInfo,
  pub last_seen: Instant,
  pub is_connected: bool,
  pub connection_count: u32,
}

impl ManagedPeer {
  pub fn new(info: PeerInfo) -> Self {
    Self {
      info,
      last_seen: Instant::now(),
      is_connected: true,
      connection_count: 1,
    }
  }

  pub fn update(&mut self, new_info: PeerInfo) -> bool {
    let metadata_changed = self.info.is_metadata_different_from(&new_info);
    self.info = new_info;
    self.last_seen = Instant::now();
    self.is_connected = true;
    self.connection_count += 1;
    metadata_changed
  }
}

#[derive(Debug, Clone)]
pub struct PeerManager {
  active_peers: Arc<RwLock<HashMap<String, ManagedPeer>>>,
  known_peers: Arc<RwLock<HashSet<String>>>,
  event_tx: mpsc::Sender<PeerEvent>,
}

impl PeerManager {
  pub fn new(event_tx: mpsc::Sender<PeerEvent>) -> Self {
    Self {
      active_peers: Arc::new(RwLock::new(HashMap::new())),
      known_peers: Arc::new(RwLock::new(HashSet::new())),
      event_tx,
    }
  }

  pub async fn upsert_peer(&self, source: &str, peer_info: PeerInfo) {
    let mut active_peers = self.active_peers.write().await;
    let mut known_peers = self.known_peers.write().await;
    let peer_id = peer_info.id.clone();

    known_peers.insert(peer_id.clone());

    match active_peers.get_mut(&peer_id) {
      Some(peer) => {
        let was_connected = peer.is_connected;
        let metadata_changed = peer.update(peer_info.clone());

        if !was_connected {
          let _ = self
            .event_tx
            .send(PeerEvent::new_reconnected(source, peer_info.clone()))
            .await;
        } else if metadata_changed {
          let _ = self
            .event_tx
            .send(PeerEvent::new_updated(
              source,
              peer_info.clone(),
              "metadata_changed",
            ))
            .await;
        }
      }
      None => {
        active_peers.insert(peer_id.clone(), ManagedPeer::new(peer_info.clone()));
        let _ = self
          .event_tx
          .send(PeerEvent::new_joined(source, peer_info))
          .await;
      }
    }
  }

  pub async fn mark_disconnected(&self, source: &str, peer_id: &str) {
    let mut peers = self.active_peers.write().await;
    if let Some(peer) = peers.get_mut(peer_id) {
      peer.is_connected = false;
      let _ = self
        .event_tx
        .send(PeerEvent::new_left(source, peer_id.to_string()))
        .await;
    }
  }

  pub async fn get_peer_info(&self, peer_id: &str) -> Option<PeerInfo> {
    let peers = self.active_peers.read().await;
    peers.get(peer_id).map(|p| p.info.clone())
  }

  pub async fn get_all_peers(&self) -> Vec<PeerInfo> {
    let peers = self.active_peers.read().await;
    peers.values().map(|p| p.info.clone()).collect()
  }

  pub async fn update_peer_metadata(
    &self,
    source: &str,
    peer_id: &str,
    new_metadata: PeerMetadata,
  ) {
    let mut peers = self.active_peers.write().await;
    if let Some(peer) = peers.get_mut(peer_id) {
      let new_info = PeerInfo {
        id: peer_id.to_string(),
        metadata: Some(new_metadata),
      };

      if peer.info.is_metadata_different_from(&new_info) {
        peer.info = new_info.clone();
        let _ = self
          .event_tx
          .send(PeerEvent::new_updated(source, new_info, "manual_update"))
          .await;
      }
    }
  }

  pub async fn check_stale_peers(&self, threshold: Duration) -> Vec<String> {
    let now = Instant::now();
    let peers = self.active_peers.read().await;

    peers
      .iter()
      .filter(|(_, peer)| peer.is_connected && now.duration_since(peer.last_seen) > threshold)
      .map(|(id, _)| id.clone())
      .collect()
  }
}
