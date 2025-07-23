use crate::peer::PeerMap;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;

/// Attempts to add a new peer to the peer map.
/// Returns `true` if the peer was newly added, `false` if it already existed.
pub async fn handle_new_peer(
    peer_id: String,
    addr: SocketAddr,
    peers: PeerMap,
    advertise_tx: &UnboundedSender<()>,
) -> anyhow::Result<bool> {
    let mut peers_map = peers.write().await;

    if peers_map.contains_key(&peer_id) {
        log::debug!("Peer {peer_id} already exists, skipping update.");
        return Ok(false);
    }

    log::debug!("Adding new peer {peer_id} at {addr} to peer map.");
    peers_map.insert(peer_id.clone(), addr);

    // Drop lock before sending
    drop(peers_map);

    // Notify the main loop
    advertise_tx.send(()).ok();

    Ok(true)
}
