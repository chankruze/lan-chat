use crate::peer::PeerMap;
use futures_util::stream::StreamExt;
use mdns::{RecordKind, Response};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

/// Starts mDNS discovery and updates the shared peer map asynchronously.
///
/// # Arguments
/// * `our_id` - The local peer's unique ID
/// * `peers` - Shared peer map (protected by Arc<RwLock<_>>)
pub async fn discover(our_id: String, peers: PeerMap) -> anyhow::Result<()> {
    let stream = mdns::discover::all("_lan-chat._tcp.local", Duration::from_secs(15))?.listen();
    tokio::pin!(stream);

    let mut seen = HashSet::new();

    while let Some(Ok(response)) = stream.next().await {
        if let Some((peer_id, addr)) = extract_peer_info(&response, &our_id) {
            if seen.insert(peer_id.clone()) {
                peers.write().await.insert(peer_id, addr);
            }
        }
    }

    Ok(())
}

fn extract_peer_info(resp: &Response, our_id: &str) -> Option<(String, std::net::SocketAddr)> {
    let peer_id = resp.records().find_map(|r| {
        if let RecordKind::TXT(txt) = &r.kind {
            txt.iter()
                .position(|v| v == "peer_id")
                .and_then(|i| txt.get(i + 1))
                .cloned()
        } else {
            None
        }
    })?;

    log::debug!("Found peer ID: {peer_id}");

    // Ignore our own ID to prevent self-discovery
    if peer_id == our_id {
        log::debug!("[mDNS] Ignoring own ID: {our_id}");
        // Return None to skip adding ourselves to the peer map
        return None;
    }

    let peer_name = resp.records().find_map(|r| match &r.kind {
        RecordKind::PTR(name) => Some(name.split('.').next()?.to_string()),
        _ => None,
    })?;

    let ip = resp.records().find_map(|r| match &r.kind {
        RecordKind::A(ip) => Some(IpAddr::V4(*ip)),
        RecordKind::AAAA(ip) => Some(IpAddr::V6(*ip)),
        _ => None,
    })?;

    let port = resp.records().find_map(|r| {
        if let RecordKind::SRV { port, .. } = r.kind {
            Some(port)
        } else {
            None
        }
    })?;

    // log the discovered peer
    log::info!(
        "\n\
    🔍 Discovered Peer\n\
     ├─ Name: {peer_name}\n\
     ├─ ID  : {peer_id}\n\
     └─ Addr: {ip}:{port}"
    );

    // Create a SocketAddr from the IP and port
    let addr = SocketAddr::new(ip, port);

    // Return the peer name and address
    Some((peer_name, addr))
}
