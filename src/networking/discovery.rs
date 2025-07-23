use crate::peer::metadata::PeerMetadata;
use crate::peer::{PeerInfo, PeerMap, handle_new_peer};
use crate::utils::parse_txt_record;
use futures_util::stream::StreamExt;
use mdns::{RecordKind, Response};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;

/// Starts mDNS discovery and updates the shared peer map asynchronously.
///
/// # Arguments
/// * `our_id` - The local peer's unique ID
/// * `peers` - Shared peer map (protected by Arc<RwLock<_>>)
pub async fn discover(
    our_id: String,
    peers: PeerMap,
    advertise_tx: UnboundedSender<()>,
) -> anyhow::Result<()> {
    let stream = mdns::discover::all("_lan-chat._tcp.local", Duration::from_secs(15))?.listen();
    tokio::pin!(stream);

    while let Some(Ok(response)) = stream.next().await {
        if let Some(peer_info) = extract_peer_info(&response, &our_id) {
            let _ = handle_new_peer(peer_info, peers.clone(), &advertise_tx).await?;
        }
    }

    Ok(())
}

fn extract_peer_info(resp: &Response, our_id: &str) -> Option<PeerInfo> {
    let txt_map: HashMap<String, String> = resp.records().find_map(|r| {
        if let RecordKind::TXT(txt) = &r.kind {
            Some(parse_txt_record(txt))
        } else {
            None
        }
    })?;

    let peer_id = txt_map.get("peer_id")?.clone();

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

    Some(PeerInfo {
        id: peer_id,
        metadata: PeerMetadata {
            addr,
            name: peer_name,
            instance,
            platform,
            version,
        },
    })
}
