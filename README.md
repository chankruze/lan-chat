### File structure

| File           | Description             |
| -------------- | ----------------------- |
| `main.rs`      | Entry point             |
| `discovery.rs` | Discovery & advertising |
| `ws_server.rs` | WebSocket host logic    |
| `ws_client.rs` | Connect to others       |
| `terminal.rs`  | Text input/output UI    |

### Packages

| Crate                   | Usage                                                                                                                                                                                                                  |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`tokio`**             | Asynchronous runtime for Rust. Used to run async tasks such as WebSocket communication, mDNS discovery, and terminal I/O concurrently. The `full` feature enables all Tokio components (e.g., TCP, time, I/O, macros). |
| **`tokio-tungstenite`** | Integrates WebSocket support (`tungstenite`) with Tokio's async runtime. Used for handling async WebSocket server and client communication.                                                                            |
| **`tungstenite`**       | A WebSocket library (low-level). Provides core WebSocket protocol implementation used by `tokio-tungstenite` under the hood.                                                                                           |
| **`futures`**           | Provides combinators and tools to work with async streams and tasks. Used for things like select! loops, stream handling, and futures chaining.                                                                        |
| **`mdns`**              | Enables multicast DNS (mDNS) service discovery on the local network. Used for peer discovery—advertising your chat instance and discovering others automatically.                                                      |
| **`anyhow`**            | Simplifies error handling in Rust. Used to return and propagate errors easily using `?` operator without declaring explicit error types.                                                                               |
| **`uuid`**              | Used to generate unique peer IDs (e.g., `Uuid::new_v4()`), which can help identify devices in the chat room and prevent message self-looping.                                                                          |
| **`whoami`**            | Gets system-level user info like username, hostname, etc. Useful for showing a default nickname or peer identity in the chat UI.                                                                                       |

### Peers on the same LAN should be able to:

📢 Broadcast their presence over mDNS.
🔍 Discover other peers on the LAN using mDNS.
🔌 Establish WebSocket connections to discovered peers.
💬 Exchange messages in real time.

### Next steps

- [ ] Implement mDNS advertisement
- [ ] Launch the WebSocket server
- [ ] Connect to discovered peers
- [ ] Enable full chat message loop with JSON encoding

### References

- https://medium.com/@potto_94870/understand-mdns-with-an-example-1e05ef70013b
