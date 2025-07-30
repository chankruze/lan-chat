use super::PeerManager;
use std::{sync::Arc, time::Duration};

const STALE_THRESHOLD: Duration = Duration::from_secs(30);

pub async fn start_health_check(manager: Arc<PeerManager>) {
  let mut interval = tokio::time::interval(Duration::from_secs(10));

  loop {
    interval.tick().await;

    let stale_peers = manager.check_stale_peers(STALE_THRESHOLD).await;

    for peer_id in stale_peers {
      manager.mark_disconnected("health_check", &peer_id).await;
    }
  }
}
