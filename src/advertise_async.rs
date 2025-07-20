use env_logger::Builder;
use libmdns::{Responder, Service};
use log::LevelFilter;
use std::env;
use tokio::signal;

/// Initializes the logger to filter logs only from your crate or selected modules.
fn init_logger() {
    let mut builder = Builder::new();

    // Show only logs from your own crate/module
    builder.filter_module("lan_chat", LevelFilter::Debug);

    // You can include libmdns logs if needed (optional)
    builder.filter_module("libmdns", LevelFilter::Info);

    // Allow override with RUST_LOG
    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    }

    let _ = builder.is_test(false).try_init();
}

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
    // Initialize logger once
    init_logger();

    let responder = Responder::new().expect("❌ Failed to create mDNS responder");

    // Keep the service handle alive
    let _svc: Service = responder.register(
        service_type.to_string(),
        instance_name.to_string(),
        port,
        txt_records,
    );

    log::info!("✅ Advertising mDNS service '{instance_name}' as '{service_type}' on port {port}");
    log::info!("📡 Press Ctrl+C to stop...");

    // Wait for termination signal
    if let Err(e) = signal::ctrl_c().await {
        eprintln!("❌ Failed to listen for shutdown signal: {e}");
    }

    log::info!("🛑 mDNS service shutting down...");
}
