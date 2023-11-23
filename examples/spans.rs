use tracing_subscriber::prelude::*;

use aeiou::Layer;

fn main() {
    let layer = Layer::new();

    tracing_subscriber::registry().with(layer).init();

    tracing::info!("a message");
    let _span1 = tracing::info_span!("span.info", mog = 4, gom = "cow").entered();
    tracing::debug!(field = "value", "my message");
    let _span2 = tracing::trace_span!("span.trace").entered();
    tracing::error!(field = "only one");
}
