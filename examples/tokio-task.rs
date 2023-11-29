use std::time::Duration;

use tracing_subscriber::prelude::*;

use ari_subscriber::Layer;

#[tokio::main]
async fn main() {
    let layer = Layer::new();

    tracing_subscriber::registry().with(layer).init();

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
    })
    .await
    .unwrap();
}
