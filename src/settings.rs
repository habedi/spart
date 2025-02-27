//! Internal settings for Spart.
//!
//! This module initializes the logging configuration for Spart at startup.
//! The logging behavior is controlled by the `DEBUG_SPART` environment variable.
//! If `DEBUG_SPART` is not set or is set to a falsy value ("0", "false", or empty),
//! logging will remain disabled. Otherwise, logging is enabled with a maximum level of DEBUG.

use ctor::ctor;
use tracing::Level;

#[ctor]
fn set_debug_level() {
    // If DEBUG_SPART is not set or set to a falsy value, disable logging.
    // Otherwise, initialize a debug-level subscriber.
    if std::env::var("DEBUG_SPART").map_or(true, |v| v == "0" || v == "false" || v.is_empty()) {
        // Option 1: Do nothing (logging macros will not output)
        // Option 2: Install a no-op subscriber to explicitly disable logging:
        // tracing::subscriber::set_global_default(tracing::subscriber::NoSubscriber::default())
        //     .expect("Failed to set no-op subscriber");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
}
