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

    // Looks up the current peer in the peers_map by its id.
    // Returns Some(existing_peer_info) if found, or None if it's a new peer.
    match peers_map.get(&peer_info.id) {
        // If the peer already exists and the metadata is not different
        // 1. Log that there's nothing to update.
        // 2. Return false to indicate no update was performed.
        Some(existing_peer_info)
            if !peer_info
                .metadata
                .is_different(&existing_peer_info.metadata) =>
        {
            log::debug!("Nothing to update for peer {}", peer_info.id);
            return Ok(false);
        }
        // If the peer already exists but the metadata is different or the peer is new
        // It logs the new or updated peer using PeerInfo::log() for structured debug output.
        // Then inserts the peer info into the map.
        _ => {
            peer_info.log();
            peers_map.insert(peer_info.id.clone(), peer_info.clone());
        }
    }

    // Drop the lock before notifying
    drop(peers_map);

    // Notify the main loop
    advertise_tx.send(()).ok();

    Ok(true)
}
