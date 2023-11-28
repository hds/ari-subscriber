//! The *Async Executor Instrumentation Observability Utility* is a [`tracing-subscriber`]
//! optimized for visually debugging Tokio tracing instrumentation.
//!
//! This crate provides a [`Layer`] which writes [`tracing`] information to `stdout`. It colorizes
//! the traces that result from the tracing instrumentation in Tokio to make identifying them
//! easier.
//!
//! # Usage
//!
//! This example will set up a formatting [`tracing_subscriber::Layer`] which is then added to the
//! registry. The output from the task spawned afterwards will be seen in `stdout`.
//!
//! ```rust
//! use tracing_subscriber::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let layer = aeiou::layer();
//!
//!     tracing_subscriber::registry().with(layer).init();
//!
//!     tokio::spawn(async {
//!         tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//!     })
//!     .await
//!     .unwrap();
//! }
//! ````
//!
//! A common use case is to use `aeiou` together with the [`console-subscriber`], which aggregates
//! the same Tokio tracing instrumentation to be visualized in Tokio Console.
//!
//! ```rust
//! use tracing_subscriber::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let fmt_layer = aeiou::layer();
//!     let console_layer = console_subscriber::spawn();
//!
//!     tracing_subscriber::registry()
//!         .with(fmt_layer)
//!         .with(console_layer)
//!         .init();
//!
//!     tokio::spawn(async {
//!         tokio::time::sleep(std::time::Duration::from_millis(100)).await;
//!     })
//!     .await
//!     .unwrap();
//! }
//! ````
//!
//! ## Example output
//!
//! The beginning of the output of the above program would be:
//!
//! <pre style="background-color: #000; color: #fff">
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.746508Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <b><u><span style='color:#5aba84'>new</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.747110Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <b><u><span style='color:#5aba84'>enter</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.747340Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <b><u><span style='color:#df5853'>new</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.747539Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <b><u><span style='color:#df5853'>enter</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.747683Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <b><span style='color:#ff4d6d'>runtime::resource::state_update</span></b>: <span style='color:#c9184a'>duration=101, duration.unit=&quot;ms&quot;, duration.op=&quot;override&quot;</span>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.747854Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <span style='color:#5c8dce'>runtime.resource.async_op[<b><span style='color:#508ee3'>274877906946</span></b></span><span style='color:#5c8dce'>]{source=&quot;Sleep::new_timeout&quot;}</span> <b><u><span style='color:#508ee3'>new</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.747991Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <b><u><span style='color:#df5853'>exit</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.748118Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <span style='color:#5c8dce'>runtime.resource.async_op[<b><span style='color:#508ee3'>274877906946</span></b></span><span style='color:#5c8dce'>]{source=&quot;Sleep::new_timeout&quot;}</span> <b><u><span style='color:#508ee3'>enter</span></u></b>
//! <span style='opacity:0.67'><b><span style='color:#aaa'>2023-11-28</span></b></span><span style='opacity:0.67'>T<b><span style='color:#aaa'>10:06:44</span></b></span><span style='opacity:0.67'>.748196Z</span> <span style='color:#9d4edd'>TRACE</span> <span style='color:#489e6c'>runtime.spawn[<b><span style='color:#5aba84'>1</span></b></span><span style='color:#489e6c'>]{kind=task, task.name=, task.id=18, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=14, loc.col=5}</span> <span style='color:#ba5a57'>runtime.resource[<b><span style='color:#df5853'>274877906945</span></b></span><span style='color:#ba5a57'>]{concrete_type=&quot;Sleep&quot;, kind=&quot;timer&quot;, loc.file=&quot;examples/tokio-task.rs&quot;, loc.line=15, loc.col=9}</span> <span style='color:#5c8dce'>runtime.resource.async_op[<b><span style='color:#508ee3'>274877906946</span></b></span><span style='color:#5c8dce'>]{source=&quot;Sleep::new_timeout&quot;}</span> <span style='color:#e5e44d'>runtime.resource.async_op.poll[<b><span style='color:#f5f466'>274877906947</span></b></span><span style='color:#e5e44d'>]{}</span> <b><u><span style='color:#f5f466'>new</span></u></b>
//! </pre>
//!
//! ## Comparison with `tracing-subscriber`
//!
//! `aeiou` is built on top of `tracing-subscriber` and uses its registry (as do the majority of
//! `tracing` subscribers). It offers an alternative to the [`fmt::Subscriber`] and underlying
//! [`fmt::Layer`] in that crate.
//!
//! If you are in doubt about which format subscriber to use, pick the one from
//! `tracing-subscriber`. It is more flexible and without a doubt, much more performant.
//!
//! You would only use the `aeiou` format [`Layer`] if you have a specific need to visualize the
//! tracing instrumentation built into Tokio.
//!
//! ## Supported Rust Versions
//!
//! `aeiou` is built against the latest stable release. The minimum supported version is 1.64. The
//! current version of `aeiou` is not guaranteed to build on Rust versions earlier than the
//! minimum supported version.
//!
//! ## License
//!
//! This project is licensed under the [MIT license].
//!
//! [MIT license]: https://github.com/hds/aeiou/blob/main/LICENSE
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
//! in `aeiou` by you, shall be licensed as MIT, without any additional terms or conditions.
//!
//! [`console-subscriber`]: https://docs.rs/console-subscriber/latest/console_subscriber/
//! [`tracing-subscriber`]: tracing_subscriber
//! [`fmt::Layer`]: struct@tracing_subscriber::fmt::Layer
//! [`fmt::Subscriber`]: struct@tracing_subscriber::fmt::Subscriber
#![deny(rustdoc::missing_crate_level_docs, missing_docs)]

pub(crate) mod fmt;
mod layer;

pub use layer::{layer, Layer};
