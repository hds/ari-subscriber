use tracing_subscriber::prelude::*;

use aeiou::Layer;

fn main() {
    let layer = Layer::new();

    tracing_subscriber::registry().with(layer).init();

    tracing::trace!("This is way too verbose");
    tracing::debug!(field = "value", "my message");
    tracing::info!("a message");
    tracing::warn!("warn me!");
    tracing::error!(field = "only one");
}
