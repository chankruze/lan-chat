use crate::config;
use crate::peer::PeerInfo;
use crate::peer::metadata::PeerMetadata;
use crate::utils::parse_txt_record;
use mdns::{RecordKind, Response};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};

pub fn extract_peer_info(resp: &Response, our_id: &str) -> Option<PeerInfo> {
  let txt_map: HashMap<String, String> = resp.records().find_map(|r| {
    if let RecordKind::TXT(txt) = &r.kind {
      Some(parse_txt_record(txt))
    } else {
      None
    }
  })?;

  let peer_id = txt_map.get("peer_id")?.clone();

  if peer_id == our_id {
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

  let mdns_addr = SocketAddr::new(ip, port);
  let ws_addr = SocketAddr::new(ip, config::WS_PORT);

  Some(PeerInfo {
    id: peer_id,
    metadata: Some(PeerMetadata {
      mdns_addr,
      ws_addr,
      name: peer_name,
      instance,
      platform,
      version,
    }),
  })
}
