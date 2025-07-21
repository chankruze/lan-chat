use crate::config;
use hostname::get as get_hostname;
use std::collections::HashMap;
use std::vec::Vec;

pub fn generate_peer_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_peer_name() -> String {
    whoami::username()
}

pub fn generate_instance_name(peer_name: &str, peer_id: &str) -> String {
    let short_id = &peer_id[..6];
    let hostname = get_hostname()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown-host".to_string());

    format!("{peer_name}@{hostname} [{short_id}]")
}

/// Parses TXT records (["key=value", ...]) into a key-value map.
pub fn parse_txt_record(txt: &[String]) -> HashMap<String, String> {
    txt.iter()
        .filter_map(|entry| {
            let mut parts = entry.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}

/// Builds mDNS TXT records as a list of `&str` from given metadata.
pub fn build_txt_records(peer_id: &str, peer_name: &str, instance_name: &str) -> Vec<String> {
    vec![
        format!("{}={}", config::TXT_KEY_PEER_ID, peer_id),
        format!("{}={}", config::TXT_KEY_PEER_NAME, peer_name),
        format!("{}={}", config::TXT_KEY_INSTANCE, instance_name),
        format!("{}={}", config::TXT_KEY_PLATFORM, whoami::platform()),
        format!("{}={}", config::TXT_KEY_VERSION, env!("CARGO_PKG_VERSION")),
    ]
}
