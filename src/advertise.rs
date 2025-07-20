use libmdns::{Responder, Service};
use std::{thread, time::Duration};

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
/// advertise::start_mdns_service("_http._tcp", "My Rust Service", 8080, &["path=/"]);
/// ```
pub fn start_mdns_service(
    service_type: &str,
    instance_name: &str,
    port: u16,
    txt_records: &[&str],
) {
    // Optional: enable logging
    let mut builder = env_logger::Builder::new();
    builder.parse_filters("libmdns=debug");
    let _ = builder.try_init(); // prevent double-init panic

    // Start the responder and register the service
    let responder = Responder::new().expect("Failed to create mDNS responder");
    let _svc: Service = responder.register(
        service_type.to_owned(),
        instance_name.to_owned(),
        port,
        txt_records,
    );

    println!(
        "Advertising mDNS service '{}' as {} on port {}",
        instance_name, service_type, port
    );

    // Keep the service running
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}
