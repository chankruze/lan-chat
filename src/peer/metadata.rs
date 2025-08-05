use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerMetadata {
  pub mdns_addr: SocketAddr,
  pub ws_addr: SocketAddr,
  pub name: String,
  pub instance: String,
  pub version: String,
  pub platform: String,
}

impl PeerMetadata {
  pub fn is_different(&self, other: &Self) -> bool {
    self.mdns_addr != other.mdns_addr
      || self.ws_addr != other.ws_addr
      || self.name != other.name
      || self.instance != other.instance
      || self.version != other.version
      || self.platform != other.platform
  }

  pub fn log(&self) {
    log::debug!("\n{self:#?}");
  }
}
