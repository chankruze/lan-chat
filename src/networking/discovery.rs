use super::utils::extract_peer_info;
use crate::peer::PeerManager;
use futures_util::stream::StreamExt;
use std::{sync::Arc, time::Duration};

pub async fn discover(manager: Arc<PeerManager>, our_id: String) -> anyhow::Result<()> {
  let stream = mdns::discover::all("_lan-chat._tcp.local", Duration::from_secs(15))?.listen();
  tokio::pin!(stream);

  while let Some(response) = stream.next().await {
    if let Some(peer_info) = extract_peer_info(&response.unwrap(), &our_id) {
      if peer_info.id != our_id {
        manager.upsert_peer("mdns", peer_info).await;
      }
    }
  }

  Ok(())
}
