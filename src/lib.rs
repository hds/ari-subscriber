//! A [`tracing-subscriber`] optimized for visually debugging Tokio tracing instrumentation.
//!
//! This crate provides a [`Layer`] which writes [`tracing`] information to `stdout`. It marks the
//! traces that result from the tracing instrumentation in Tokio to make identifying them easier.
//!
//! [`tracing-subscriber`]: tracing_subscriber
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
#![deny(rustdoc::missing_crate_level_docs, missing_docs)]

pub(crate) mod fmt;
mod layer;

pub use layer::{layer, Layer};
