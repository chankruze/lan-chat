use super::WsManager;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

pub async fn start_websocket_server(
  addr: SocketAddr,
  ws_manager: Arc<WsManager>,
) -> anyhow::Result<()> {
  // Check if server is already running
  if ws_manager.is_server_running() {
    if let Some(current_addr) = ws_manager.get_server_address().await {
      if current_addr == addr {
        log::info!("WebSocket server already running on {addr}");
        return Ok(());
      } else {
        return Err(anyhow::anyhow!(
          "WebSocket server already running on different address: {current_addr}"
        ));
      }
    }
  }

  let listener = TcpListener::bind(addr).await?;
  ws_manager.set_server_state(true, Some(addr)).await;
  println!("WebSocket server listening on: {addr}");

  let result = async {
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
  }.await;

  // Clean up server state when the server stops
  ws_manager.set_server_state(false, None).await;
  result
}

/// Start WebSocket server if not already running
pub async fn ensure_websocket_server(
  addr: SocketAddr,
  ws_manager: Arc<WsManager>,
) -> anyhow::Result<()> {
  if ws_manager.is_server_running() {
    log::info!("WebSocket server already running");
    return Ok(());
  }

  let ws_manager_clone = ws_manager.clone();
  tokio::spawn(async move {
    if let Err(e) = start_websocket_server(addr, ws_manager_clone).await {
      log::error!("Failed to start WebSocket server: {e}");
    }
  });

  // Give the server a moment to start
  tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
  Ok(())
}
