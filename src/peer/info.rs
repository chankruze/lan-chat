use super::PeerMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PeerMetadata>,
}

impl PeerInfo {
    pub fn new(id: impl Into<String>, metadata: PeerMetadata) -> Self {
        Self {
            id: id.into(),
            metadata: Some(metadata),
        }
    }

    pub fn without_metadata(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            metadata: None,
        }
    }

    pub fn is_metadata_different_from(&self, other: &Self) -> bool {
        match (&self.metadata, &other.metadata) {
            (Some(a), Some(b)) => a.is_different(b),
            (None, None) => false,
            _ => true,
        }
    }

    pub fn log(&self) {
        log::debug!("\n{self:#?}");
    }
}
