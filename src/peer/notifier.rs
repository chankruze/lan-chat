use crate::peer::PeerEvent;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct PeerNotifier {
  advertise_tx: UnboundedSender<()>,
  event_tx: UnboundedSender<PeerEvent>,
}

impl PeerNotifier {
  pub fn new(advertise_tx: UnboundedSender<()>, event_tx: UnboundedSender<PeerEvent>) -> Self {
    Self {
      advertise_tx,
      event_tx,
    }
  }

  pub fn advertise(&self) {
    let _ = self.advertise_tx.send(());
  }

  pub fn emit_event(&self, event: PeerEvent) {
    let _ = self.event_tx.send(event);
  }
}
