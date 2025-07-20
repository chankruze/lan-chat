use crate::peer::PeerMap;
use crate::utils::parse_txt_record;
use futures_util::stream::StreamExt;
use mdns::{RecordKind, Response};
use std::collections::{HashMap, HashSet};
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

fn extract_peer_info(resp: &Response, our_id: &str) -> Option<(String, SocketAddr)> {
    let txt_map: HashMap<String, String> = resp.records().find_map(|r| {
        if let RecordKind::TXT(txt) = &r.kind {
            Some(parse_txt_record(txt))
        } else {
            None
        }
    })?;

    let peer_id = txt_map.get("peer_id")?;

    if peer_id == our_id {
        log::debug!("Ignoring own ID: {our_id}");
        return None;
    }

    let peer_name = txt_map.get("peer_name")?.clone();
    let instance = txt_map.get("instance")?.clone();
    let platform = txt_map.get("platform")?.clone();
    let version = txt_map.get("version")?.clone();

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

    let addr = SocketAddr::new(ip, port);

    log::info!(
        "\n\
    🔍 Discovered Peer\n\
     ├─ Name     : {peer_name}\n\
     ├─ ID       : {peer_id}\n\
     ├─ Instance : {instance}\n\
     ├─ Platform : {platform}\n\
     ├─ Version  : {version}\n\
     └─ Addr     : {ip}:{port}"
    );

    Some((peer_name, addr))
}
