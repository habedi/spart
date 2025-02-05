use ctor::ctor;
use tracing::Level;
use tracing_subscriber;

#[ctor]
fn set_debug_level() {
    // If DEBUG_QUADTREE_ZNG is not set or set to false, disable logging. Otherwise, enable logging
    if std::env::var("DEBUG_SPART").map_or(true, |v| v == "0" || v == "false" || v.is_empty()) {
        // Disable logging
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }

    println!("DEBUG_SPART: {:?}", std::env::var("DEBUG_SPART"));
}
