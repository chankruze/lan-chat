use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type PeerMap = Arc<RwLock<HashMap<String, SocketAddr>>>;
