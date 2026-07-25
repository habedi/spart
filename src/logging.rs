//! ## Logging Configuration for Spart
//!
//! This module initializes the logging configuration for Spart at startup.
//! The logging behavior is controlled by the `DEBUG_SPART` environment variable.
//! If `DEBUG_SPART` is not set or is set to a falsy value ("0", "false", or empty),
//! logging will remain disabled. Otherwise, logging is enabled with a maximum level of DEBUG.

#[cfg(feature = "setup_tracing")]
use ctor::ctor;
#[cfg(feature = "setup_tracing")]
use tracing::Level;

// `ctor` runs this before `main`, which is inherently unsafe (it touches process-global state before
// the runtime is up), so the attribute has to say so explicitly.
#[cfg(feature = "setup_tracing")]
#[ctor(unsafe)]
fn set_debug_level() {
    // If DEBUG_SPART is not set or set to a falsy value, disable logging.
    // Otherwise, initialize a debug-level subscriber.
    if std::env::var("DEBUG_SPART").map_or(true, |v| v == "0" || v == "false" || v.is_empty()) {
        // Option 1: Do nothing (logging macros will not output)
        // Option 2: Install a no-op subscriber to explicitly disable logging:
        let _ =
            tracing::subscriber::set_global_default(tracing::subscriber::NoSubscriber::default());
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
}
