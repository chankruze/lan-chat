use super::{PeerInfo, PeerMetadata};

// This enables us to convert a PeerInfo struct into a tuple of (String, PeerMetadata).
//
// example:
// let info = PeerInfo {
//     id: "abc123".into(),
//     metadata: PeerMetadata { /* ... */ },
// };
// let (id, metadata): (String, PeerMetadata) = info.into();
//
// This is helpful when we want to store or serialize peers as key-value pairs (e.g., in a HashMap<String, PeerMetadata>).
impl From<PeerInfo> for (String, PeerMetadata) {
  fn from(info: PeerInfo) -> Self {
    (info.id, info.metadata.expect("metadata should exist here"))
  }
}

// This allows us to create a PeerInfo from a borrowed ID (&str) and metadata reference (&PeerMetadata).
//
// example:
// let id = "abc123";
// let meta = PeerMetadata { /* ... */ };
// let info: PeerInfo = (id, &meta).into();
//
// This is useful when reading from a map (HashMap<String, PeerMetadata>) and we want to recreate a PeerInfo from its entry without moving values.
impl From<(&str, &PeerMetadata)> for PeerInfo {
  fn from((id, meta): (&str, &PeerMetadata)) -> Self {
    PeerInfo {
      id: id.to_string(),
      metadata: Some(meta.clone()),
    }
  }
}
