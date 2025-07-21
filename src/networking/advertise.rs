use crate::{config, utils};
use libmdns::{Responder, Service};
use tokio::sync::mpsc::UnboundedReceiver;

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
/// advertise::start_mdns_service_with_re_advertise("1234", "Alice", "alice@mac", rx).await;
/// // You can call `tx.send(())` to trigger re-advertisement.
/// ```
pub async fn start_mdns_service_with_re_advertise(
    peer_id: &str,
    peer_name: &str,
    instance_name: &str,
    mut advertise_rx: UnboundedReceiver<()>,
) {
    let txt_strings = utils::build_txt_records(peer_id, peer_name, instance_name);
    let txt_records: Vec<&str> = txt_strings.iter().map(|s| s.as_str()).collect();

    let responder = Responder::new().expect("❌ Failed to create mDNS responder");

    log::info!("📡 Starting mDNS service advertiser with re-advertise support...");

    let shutdown = tokio::signal::ctrl_c();
    tokio::pin!(shutdown);

    loop {
        // (Re)register the service
        let _svc: Service = responder.register(
            config::SERVICE_TYPE.to_string(),
            instance_name.to_string(),
            config::SERVICE_PORT,
            &txt_records,
        );

        log::info!("🔁 mDNS service (re)advertised");

        tokio::select! {
            _ = advertise_rx.recv() => {
                log::info!("🔔 Re-advertise triggered by peer discovery");
                continue; // re-register
            },
            _ = &mut shutdown => {
                log::info!("🛑 Shutting down mDNS service...");
                break;
            }
        }
    }
}
