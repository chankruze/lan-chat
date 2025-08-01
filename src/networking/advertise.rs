use crate::{config, peer::PeerIdentity, utils};
use libmdns::{Responder, Service};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc::Receiver};

/// Starts an mDNS advertisement for a LAN chat peer with support for re-advertising.
///
/// This async function continuously runs an mDNS service for a peer, using identity data
/// from shared state. It supports re-advertising via a channel trigger (e.g., on peer updates).
///
/// The service stays active until a CTRL+C shutdown signal is received.
///
/// # Arguments
///
/// * `identity` - Shared peer identity wrapped in `Arc<RwLock<PeerIdentity>>`
/// * `advertise_rx` - A `tokio::mpsc::Receiver<()>` used to trigger re-advertisement
///
/// # Behavior
///
/// - Registers an mDNS service with configured type and port.
/// - Populates TXT records with peer ID, name, instance name, platform, and version.
/// - On receiving a message via `advertise_rx`, re-advertises the service.
/// - Gracefully exits on receiving a CTRL+C signal.
///
/// # Example
///
/// ```rust
/// let (tx, rx) = tokio::sync::mpsc::channel(32);
/// let identity = Arc::new(RwLock::new(PeerIdentity::load_or_generate()));
///
/// tokio::spawn(start_mdns_service_with_re_advertise(identity.clone(), rx));
///
/// // Trigger a re-advertise later with:
/// tx.send(()).await.unwrap();
/// ```
pub async fn start_mdns_service_with_re_advertise(
  identity: Arc<RwLock<PeerIdentity>>,
  mut advertise_rx: Receiver<()>,
) {
  let responder = Responder::new().expect("❌ Failed to create mDNS responder");

  log::info!("📡 Starting mDNS service advertiser with re-advertise support...");

  let shutdown = tokio::signal::ctrl_c();
  tokio::pin!(shutdown);

  loop {
    let (peer_id, peer_name, instance_name) = {
      let identity = identity.read().await;
      (
        identity.peer_id.clone(),
        identity.peer_name.clone(),
        identity.instance_name.clone(),
      )
    }; // `identity` goes out of scope here automatically

    log::info!(
      "\n\
      🚀 Starting LAN Chat\n\
      ├─ ID      : {}\n\
      ├─ Name    : {}\n\
      ├─ Instance: {}\n\
      └─ Port    : {}",
      &peer_id,
      &peer_name,
      &instance_name,
      config::SERVICE_PORT
    );

    let txt_strings = utils::build_txt_records(&peer_id, &peer_name, &instance_name);
    let txt_records: Vec<&str> = txt_strings.iter().map(|s| s.as_str()).collect();

    // Register (or re-register) the mDNS service
    let _svc: Service = responder.register(
      config::SERVICE_TYPE.to_string(),
      instance_name,
      config::SERVICE_PORT,
      &txt_records,
    );

    log::info!("🔁 mDNS service (re)advertised");

    tokio::select! {
        _ = advertise_rx.recv() => {
            log::debug!("📣 Re-advertise signal received");
            continue;
        },
        _ = &mut shutdown => {
            log::info!("🛑 Shutting down mDNS service...");
            break;
        }
    }
  }
}
