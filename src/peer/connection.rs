pub enum Transport {
  Quic,
  Websocket,
  WebRTC,
}

pub struct ConnectionManager {
  manager: PeerManager,
  transport: Transport,
}

impl ConnectionManager {
  pub async fn send_ping(&self, peer_id: &str) -> bool {
    // Implement transport-specific ping logic
    true // or false on failure
  }
}
