use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PeerIdentity {
  pub peer_id: String,
  pub peer_name: String,

  #[serde(skip)]
  pub instance_name: String,
}

impl PeerIdentity {
  pub fn load_or_generate() -> Self {
    let path = identity_file_path();

    if let Ok(contents) = fs::read_to_string(&path) {
      if let Ok(mut identity) = serde_json::from_str::<PeerIdentity>(&contents) {
        identity.instance_name =
          utils::generate_instance_name(&identity.peer_name, &identity.peer_id);
        return identity;
      }
    }

    let default = Self::default();
    if let Some(parent) = path.parent() {
      let _ = fs::create_dir_all(parent);
    }

    if let Ok(json) = serde_json::to_string_pretty(&default) {
      let _ = fs::write(&path, json);
    }

    default
  }

  pub fn instance_name(&self) -> String {
    self.instance_name.clone()
  }

  pub fn peer_id(&self) -> String {
    self.peer_id.clone()
  }

  pub fn peer_name(&self) -> String {
    self.peer_name.clone()
  }

  pub fn update_peer_name(&mut self, new_name: &str) {
    if self.peer_name == new_name {
      return;
    }

    self.peer_name = new_name.to_string();
    self.instance_name = utils::generate_instance_name(&self.peer_name, &self.peer_id);

    let _ = self.save();
  }

  fn save(&self) -> std::io::Result<()> {
    let path = identity_file_path();
    if let Some(parent) = path.parent() {
      let _ = fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(self)?;
    fs::write(path, json)
  }

  pub fn log(&self) {
    log::debug!("\n{self:#?}");
  }
}

impl Default for PeerIdentity {
  fn default() -> Self {
    let peer_name = utils::generate_peer_name();
    let peer_id = utils::generate_peer_id();
    let instance_name = utils::generate_instance_name(&peer_name, &peer_id);

    PeerIdentity {
      peer_id,
      peer_name,
      instance_name,
    }
  }
}

fn identity_file_path() -> PathBuf {
  if let Some(dir) = dirs::data_local_dir() {
    dir.join("lan-chat").join("identity.json")
  } else {
    dirs::home_dir()
      .unwrap_or_else(|| PathBuf::from("."))
      .join(".lan-chat")
      .join("identity.json")
  }
}
