use hostname::get as get_hostname;

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
