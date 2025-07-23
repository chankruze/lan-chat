use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use when you're inside the same module hierarchy, like in a sibling module.
// super means "go up one level in the module tree."
use super::PeerInfo;

pub type PeerMap = Arc<RwLock<HashMap<String, PeerInfo>>>;
