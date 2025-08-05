use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum WsEvent {
  Connected {
    id: Uuid,
    timestamp: DateTime<Utc>,
    addr: String,
  },
  Disconnected {
    id: Uuid,
    timestamp: DateTime<Utc>,
    addr: String,
  },
  MessageReceived {
    id: Uuid,
    timestamp: DateTime<Utc>,
    addr: String,
    message: String,
  },
}

impl WsEvent {
  pub fn new_connected(addr: String) -> Self {
    Self::Connected {
      id: Uuid::new_v4(),
      timestamp: Utc::now(),
      addr,
    }
  }

  pub fn new_disconnected(addr: String) -> Self {
    Self::Disconnected {
      id: Uuid::new_v4(),
      timestamp: Utc::now(),
      addr,
    }
  }

  pub fn new_message_received(addr: String, message: String) -> Self {
    Self::MessageReceived {
      id: Uuid::new_v4(),
      timestamp: Utc::now(),
      addr,
      message,
    }
  }
}
