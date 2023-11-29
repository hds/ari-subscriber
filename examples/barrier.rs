use std::{sync::Arc, time::Duration};

use tokio::sync::Barrier;
use tracing_subscriber::prelude::*;

use ari_subscriber::Layer;

#[tokio::main]
async fn main() {
    let layer = Layer::new();

    tracing_subscriber::registry().with(layer).init();

    let barrier = Arc::new(Barrier::new(2));

    let barrier1 = barrier.clone();
    let jh = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        barrier1.wait().await;
    });

    barrier.wait().await;

    jh.await.unwrap();
}
