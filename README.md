# Async Executor Instrumentation Observability Utility (aeiou)

The *Async Executor Instrumentation Observability Utility* is a [`tracing-subscriber`]
optimized for visually debugging Tokio tracing instrumentation.

This crate provides a [`Layer`] which writes [`tracing`] information to `stdout`. It colorizes
the traces that result from the tracing instrumentation in Tokio to make identifying them
easier.

# Usage

This example will set up a formatting [`tracing_subscriber::Layer`] which is then added to the
registry. The output from the task spawned afterwards will be seen in `stdout`.

```rust
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    let layer = aeiou::layer();

    tracing_subscriber::registry().with(layer).init();

    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    })
    .await
    .unwrap();
}
````

A common use case is to use `aeiou` together with the [`console-subscriber`], which aggregates
the same Tokio tracing instrumentation to be visualized in Tokio Console.

```rust
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    let fmt_layer = aeiou::layer();
    let console_layer = console_subscriber::spawn();

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(console_layer)
        .init();

    tokio::spawn(async {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    })
    .await
    .unwrap();
}
````

## Comparison with `tracing-subscriber`

`aeiou` is built on top of `tracing-subscriber` and uses its registry (as do the majority of
`tracing` subscribers). It offers an alternative to the [`fmt::Subscriber`] and underlying
[`fmt::Layer`] in that crate.

If you are in doubt about which format subscriber to use, pick the one from
`tracing-subscriber`. It is more flexible and without a doubt, much more performant.

You would only use the `aeiou` format [`Layer`] if you have a specific need to visualize the
tracing instrumentation built into Tokio.

## Supported Rust Versions

`aeiou` is built against the latest stable release. The minimum supported version is 1.64. The
current version of `aeiou` is not guaranteed to build on Rust versions earlier than the
minimum supported version.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/hds/aeiou/blob/main/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in `aeiou` by you, shall be licensed as MIT, without any additional terms or conditions.

[`Layer`]: src/layer.rs
[`tracing`]: https://docs.rs/tracing/0.1.40/tracing/
[`console-subscriber`]: https://docs.rs/console-subscriber/latest/console_subscriber/
[`tracing-subscriber`]: tracing_subscriber
[`tracing_subscriber::Layer`]: https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/layer/trait.Layer.html
[`fmt::Layer`]: https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/fmt/struct.Layer.html
[`fmt::Subscriber`]: https://docs.rs/tracing-subscriber/0.3.18/tracing_subscriber/fmt/struct.Subscriber.html