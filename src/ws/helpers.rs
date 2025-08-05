use super::WsManager;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

pub async fn start_websocket_server(
  addr: SocketAddr,
  ws_manager: Arc<WsManager>,
) -> anyhow::Result<()> {
  let listener = TcpListener::bind(addr).await?;
  println!("WebSocket server listening on: {addr}");

  loop {
    let (stream, socket_addr) = listener.accept().await?;
    let ws_manager = ws_manager.clone();

    tokio::spawn(async move {
      match accept_async(stream).await {
        Ok(ws_stream) => {
          ws_manager.handle_connection(ws_stream, socket_addr).await;
        }
        Err(e) => {
          eprintln!("WebSocket handshake error from {socket_addr}: {e}");
          ws_manager.emit_error(socket_addr, e.to_string()).await;
        }
      }
    });
  }
}
