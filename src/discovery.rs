use crate::peer::PeerMap;
use futures_util::stream::StreamExt;
use mdns::{RecordKind, Response};
use std::collections::HashSet;
use std::net::IpAddr;
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
                println!("[mDNS] Found peer {peer_id} at {addr}");
                peers.write().await.insert(peer_id, addr);
            }
        }
    }

    Ok(())
}
fn extract_peer_info(resp: &Response, our_id: &str) -> Option<(String, std::net::SocketAddr)> {
    let peer_name = resp.records().find_map(|r| match &r.kind {
        RecordKind::PTR(name) => Some(name.split('.').next()?.to_string()),
        _ => None,
    })?;

    if peer_name == our_id {
        return None;
    }

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

    Some((peer_name, std::net::SocketAddr::new(ip, port)))
}
