use crate::{config, peer::PeerIdentity, utils};
use libmdns::{Responder, Service};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc::UnboundedReceiver};

/// Starts an mDNS advertisement for a LAN chat peer with support for re-advertising.
///
/// This function launches an mDNS service using the given peer details and registers it
/// under a predefined service type. It supports re-advertising the service when triggered
/// via the `advertise_rx` channel (e.g., due to peer discovery refresh).
///
/// The service continues to run until a CTRL+C shutdown signal is received.
///
/// # Arguments
///
/// * `peer_id` - Unique identifier for the peer (used in TXT records)
/// * `peer_name` - Human-readable name for the peer (used in TXT records)
/// * `instance_name` - mDNS instance name to register (e.g., "alice@laptop")
/// * `advertise_rx` - A channel receiver to trigger re-advertising the service
///
/// # Behavior
///
/// - Registers an mDNS service with the configured service type and port.
/// - Adds metadata via TXT records such as peer ID, name, platform, and version.
/// - Listens for re-advertise signals and re-registers the service on demand.
/// - Gracefully shuts down on CTRL+C signal.
///
/// # Example
/// ```rust
/// let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
/// let identity = PeerIdentity::load_or_generate();
/// start_mdns_service_with_re_advertise(&identity, rx).await;
/// // The service will run until a CTRL+C signal is received.
/// // To trigger re-advertisement, you can send a message on the `tx`
/// // You can call `tx.send(())` to trigger re-advertisement.
/// ```
pub async fn start_mdns_service_with_re_advertise(
  identity: Arc<RwLock<PeerIdentity>>,
  mut advertise_rx: UnboundedReceiver<()>,
) {
  let responder = Responder::new().expect("❌ Failed to create mDNS responder");

  log::info!("📡 Starting mDNS service advertiser with re-advertise support...");

  let shutdown = tokio::signal::ctrl_c();
  tokio::pin!(shutdown);

  loop {
    // let (peer_id, peer_name, instance_name) = {
    //   let identity = identity.read().await;
    //   (
    //     identity.peer_id.clone(),
    //     identity.peer_name.clone(),
    //     identity.instance_name.clone(),
    //   )
    // };

    // or

    let identity = identity.read().await;
    let peer_id = identity.peer_id.clone();
    let peer_name = identity.peer_name.clone();
    let instance_name = identity.instance_name.clone();
    drop(identity); // 🔓 Explicitly release lock

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

    // (Re)register the service
    let _svc: Service = responder.register(
      config::SERVICE_TYPE.to_string(),
      instance_name,
      config::SERVICE_PORT,
      &txt_records,
    );

    log::info!("🔁 mDNS service (re)advertised");

    tokio::select! {
        _ = advertise_rx.recv() => {
            continue;
        },
        _ = &mut shutdown => {
            log::info!("🛑 Shutting down mDNS service...");
            break;
        }
    }
  }
}
