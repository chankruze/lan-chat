use crate::config;
use libmdns::{Responder, Service};
use std::time::Duration;
use tokio::{select, signal, time::interval};

/// Starts an mDNS advertisement for a given service type, instance name, port, and optional TXT records.
///
/// # Arguments
///
/// * `service_type` - The service type (e.g., `_http._tcp`)
/// * `instance_name` - The human-readable name for the service instance
/// * `port` - The port on which the service is running
/// * `txt_records` - Optional TXT records describing the service
///
/// # Example
/// ```rust
/// advertise::start_mdns_service("_http._tcp", "My Rust Service", 8080, &["path=/"]).await;
/// ```
pub async fn start_mdns_service(
    service_type: &str,
    instance_name: &str,
    port: u16,
    txt_records: &[&str],
) {
    let responder = Responder::new().expect("❌ Failed to create mDNS responder");

    log::info!(
        "📡 Starting mDNS advertisement loop for '{instance_name}' on service '{service_type}' at port {port}"
    );
    log::info!("Press Ctrl+C to stop...");

    let mut interval = interval(Duration::from_secs(60));
    let shutdown = signal::ctrl_c();
    tokio::pin!(shutdown); // 🔧 pin it to be used inside `select!`

    loop {
        let _svc: Service = responder.register(
            service_type.to_string(),
            instance_name.to_string(),
            port,
            txt_records,
        );

        log::info!("✅ (Re)advertised mDNS service '{instance_name}'");

        select! {
            _ = interval.tick() => {
                continue; // re-register on next tick
            }
            _ = &mut shutdown => {
                log::info!("🛑 Received Ctrl+C, shutting down mDNS service...");
                break;
            }
        }
    }
}

pub async fn start_mdns_service_with_metadata(peer_id: &str, peer_name: &str, instance_name: &str) {
    let txt_strings = [
        format!("{}={}", config::TXT_KEY_PEER_ID, peer_id),
        format!("{}={}", config::TXT_KEY_PEER_NAME, peer_name),
        format!("{}={}", config::TXT_KEY_INSTANCE, instance_name),
        format!("{}={}", config::TXT_KEY_PLATFORM, whoami::platform()),
        format!("{}={}", config::TXT_KEY_VERSION, env!("CARGO_PKG_VERSION")),
    ];

    let txt_records: Vec<&str> = txt_strings.iter().map(|s| s.as_str()).collect();

    start_mdns_service(
        config::SERVICE_TYPE,
        instance_name,
        config::SERVICE_PORT,
        &txt_records,
    )
    .await;
}
