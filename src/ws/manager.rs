use super::event::WsEvent;
use futures_util::{SinkExt, StreamExt};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
  net::TcpStream,
  sync::{RwLock, mpsc::Sender},
};
use tokio_tungstenite::{WebSocketStream, connect_async, tungstenite::protocol::Message};

#[derive(Clone)]
pub struct WsManager {
  connections: Arc<RwLock<HashMap<String, tokio::sync::mpsc::UnboundedSender<Message>>>>,
  event_sender: Sender<WsEvent>,
}

impl WsManager {
  pub fn new(event_sender: Sender<WsEvent>) -> Self {
    Self {
      connections: Arc::new(RwLock::new(HashMap::new())),
      event_sender,
    }
  }

  // Handles an incoming WebSocket connection from the server side
  pub async fn handle_connection(&self, stream: WebSocketStream<TcpStream>, addr: SocketAddr) {
    let addr_str = addr.clone().to_string();
    let (mut write, mut read) = stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    self
      .connections
      .write()
      .await
      .insert(addr_str.clone(), tx.clone());

    let _ = self
      .event_sender
      .send(WsEvent::new_connected(addr_str.clone()))
      .await;

    // Clone addr_str for each task to avoid move issues
    let addr_str_write = addr_str.clone();
    let addr_str_read = addr_str.clone();

    // Write task
    tokio::spawn(async move {
      while let Some(msg) = rx.recv().await {
        if let Err(e) = write.send(msg).await {
          log::error!("WS send error to {}: {}", addr_str_write.clone(), e);
          break;
        }
      }
    });

    // Read task
    let event_sender = self.event_sender.clone();
    tokio::spawn(async move {
      while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
          let _ = event_sender
            .send(WsEvent::new_message_received(
              addr_str_read.clone(),
              text.to_string(),
            ))
            .await;
        }
      }

      // Disconnected
      let _ = event_sender
        .send(WsEvent::new_disconnected(addr_str_read.clone()))
        .await;
    });
  }

  pub async fn connect(&self, addr: String) -> anyhow::Result<()> {
    let url = format!("ws://{addr}");
    let (ws_stream, _) = connect_async(&url).await?;
    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    self
      .connections
      .write()
      .await
      .insert(addr.clone(), tx.clone());

    let event_sender = self.event_sender.clone();
    event_sender
      .send(WsEvent::new_connected(addr.clone()))
      .await?;

    // Spawn write task
    tokio::spawn(async move {
      while let Some(msg) = rx.recv().await {
        if let Err(e) = write.send(msg).await {
          log::error!("WS send error: {e}");
          break;
        }
      }
    });

    // Spawn read task
    let event_sender = self.event_sender.clone();
    let addr_clone = addr.clone();

    tokio::spawn(async move {
      while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
          let _ = event_sender
            .send(WsEvent::new_message_received(
              addr_clone.clone(),
              text.to_string(),
            ))
            .await;
        }
      }

      // Disconnected
      let _ = event_sender
        .send(WsEvent::new_disconnected(addr.clone()))
        .await;
    });

    Ok(())
  }

  pub async fn send_message(&self, addr: &str, msg: String) -> anyhow::Result<()> {
    if let Some(sender) = self.connections.read().await.get(addr) {
      sender.send(Message::Text(msg.into()))?;
      Ok(())
    } else {
      Err(anyhow::anyhow!("No active connection to {addr}"))
    }
  }

  pub async fn disconnect(&self, addr: &str) {
    self.connections.write().await.remove(addr);
    let _ = self
      .event_sender
      .send(WsEvent::new_disconnected(addr.to_string()))
      .await;
  }

  // Emits an error as a disconnected event (customize this as needed)
  pub async fn emit_error(&self, addr: SocketAddr, error: String) {
    log::error!("WebSocket error from {addr}: {error}");
    let _ = self
      .event_sender
      .send(WsEvent::new_disconnected(addr.to_string()))
      .await;
  }
}
