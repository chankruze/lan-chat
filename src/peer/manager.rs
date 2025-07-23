use crate::peer::{PeerInfo, PeerMap};
use tokio::sync::mpsc::UnboundedSender;

/// Attempts to add or update a peer in the peer map.
/// Returns `true` if the peer was newly added or updated meaningfully, `false` otherwise.
pub async fn handle_new_peer(
    peer_info: PeerInfo,
    peers: PeerMap,
    advertise_tx: &UnboundedSender<()>,
) -> anyhow::Result<bool> {
    let mut peers_map = peers.write().await;

    let should_update = match peers_map.get(&peer_info.id) {
        Some(existing_peer_info) => match (&peer_info.metadata, &existing_peer_info.metadata) {
            (Some(new), Some(old)) => new.is_different(old),
            (None, None) => false,
            _ => true,
        },
        None => true, // New peer, definitely update
    };

    if !should_update {
        log::debug!("Nothing to update for peer {}", peer_info.id);
        return Ok(false);
    }

    peer_info.log();
    peers_map.insert(peer_info.id.clone(), peer_info.clone());

    // Drop the lock before notifying
    drop(peers_map);

    // Notify the main loop
    advertise_tx.send(()).ok();

    Ok(true)
}
