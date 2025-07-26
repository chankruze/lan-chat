#[cfg(not(feature = "disable-logs"))]
use env_logger::Builder;
#[cfg(not(feature = "disable-logs"))]
use std::env;

use log::LevelFilter;

/// Initializes the logger to filter logs only from your crate or selected modules.
pub fn init_logger() {
  // If the "disable-logs" feature is enabled, disable all logging
  #[cfg(feature = "disable-logs")]
  {
    log::set_max_level(LevelFilter::Off);
    return;
  }

  // Normal logging setup when logging is enabled
  #[cfg(not(feature = "disable-logs"))]
  {
    let mut builder = Builder::new();

    builder.filter_module("lan_chat", LevelFilter::Debug);
    builder.filter_module("libmdns", LevelFilter::Debug);

    if let Ok(rust_log) = env::var("RUST_LOG") {
      builder.parse_filters(&rust_log);
    }

    let _ = builder.is_test(false).try_init();
  }
}
