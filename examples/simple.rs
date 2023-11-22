use tracing_subscriber::prelude::*;

use aeiou::Layer;

fn main() {
    let layer = Layer::new();

    tracing_subscriber::registry().with(layer).init();

    tracing::info!("a message");
    tracing::debug!(field = "value", "my message");
    tracing::error!(field = "only one");
}
