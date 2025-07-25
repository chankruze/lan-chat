use env_logger::Builder;
use log::LevelFilter;
use std::env;

/// Initializes the logger to filter logs only from your crate or selected modules.
pub fn init_logger() {
  let mut builder = Builder::new();

  // Show only logs from your own crate/module
  builder.filter_module("lan_chat", LevelFilter::Debug);

  // You can include libmdns logs if needed (optional)
  builder.filter_module("libmdns", LevelFilter::Info);

  // Allow override with RUST_LOG
  if let Ok(rust_log) = env::var("RUST_LOG") {
    builder.parse_filters(&rust_log);
  }

  let _ = builder.is_test(false).try_init();
}
