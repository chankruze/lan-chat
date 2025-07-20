use hostname::get as get_hostname;
use std::collections::HashMap;

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
