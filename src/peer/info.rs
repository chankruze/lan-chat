use super::PeerMetadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerInfo {
    pub id: String,
    pub metadata: PeerMetadata,
}

impl PeerInfo {
    pub fn new(id: impl Into<String>, metadata: PeerMetadata) -> Self {
        Self {
            id: id.into(),
            metadata,
        }
    }

    pub fn log(&self) {
        log::debug!("\n{self:#?}");
    }
}
