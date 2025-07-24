use crate::peer::PeerInfo;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum PeerEvent {
  Joined {
    id: Uuid,
    timestamp: DateTime<Utc>,
    source: String,
    peer: PeerInfo,
  },
  Updated {
    id: Uuid,
    timestamp: DateTime<Utc>,
    source: String,
    peer: PeerInfo,
  },
  Left {
    id: Uuid,
    timestamp: DateTime<Utc>,
    source: String,
    peer: PeerInfo,
  },
}

impl PeerEvent {
  pub fn new_joined(source: impl Into<String>, peer: PeerInfo) -> Self {
    Self::Joined {
      id: Uuid::new_v4(),
      timestamp: Utc::now(),
      source: source.into(),
      peer,
    }
  }

  pub fn new_updated(source: impl Into<String>, peer: PeerInfo) -> Self {
    Self::Updated {
      id: Uuid::new_v4(),
      timestamp: Utc::now(),
      source: source.into(),
      peer,
    }
  }

  pub fn new_left(source: impl Into<String>, id: String) -> Self {
    Self::Left {
      id: Uuid::new_v4(),
      timestamp: Utc::now(),
      source: source.into(),
      peer: PeerInfo { id, metadata: None },
    }
  }
}
