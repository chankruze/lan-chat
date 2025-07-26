use crate::peer::{PeerEvent, PeerInfo, PeerMap, PeerNotifier};

pub async fn handle_new_peer(
  peer_info: PeerInfo,
  peers: PeerMap,
  notifier: &PeerNotifier,
  source: &str,
) -> anyhow::Result<bool> {
  let mut peers_map = peers.write().await;

  let event = match peers_map.get(&peer_info.id) {
    Some(existing_peer_info) => {
      if peer_info.is_metadata_different_from(existing_peer_info) {
        Some(PeerEvent::new_updated(
          source.to_string(),
          peer_info.clone(),
        ))
      } else {
        return Ok(false);
      }
    }
    None => Some(PeerEvent::new_joined(source.to_string(), peer_info.clone())),
  };

  peer_info.log();
  peers_map.insert(peer_info.id.clone(), peer_info.clone());
  drop(peers_map);

  if matches!(event, Some(PeerEvent::Joined { .. })) {
    notifier.notify_advertise();
  }

  if let Some(event) = event {
    notifier.emit_event(event);
  }

  Ok(true)
}
