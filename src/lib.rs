//! # Async trait for transmitting data to peers
//!
//! `async-transmit` crate provides `Transmit` trait which allows value to be transmit asynchronously.
//!
//! This crate relies on [async-trait][], the original definition of the `Transmit` trait is:
//!
//! [async-trait]: https://github.com/dtolnay/async-trait
//!
//! ```
//! use async_trait::async_trait;
//!
//! #[async_trait]
//! pub trait Transmit<I, E> {
//!     async fn transmit(&mut self, item: I) -> Result<(), E>
//!     where
//!         I: 'async_trait;
//! }
//! ```
//!
//! So use `#[async_trait]` to when implement `Transmit` like:
//!
//! ```
//! use async_transmit::*;
//! use async_trait::async_trait;
//!
//! struct VoidTransmitter {}
//!
//! #[async_trait]
//! impl<I, E> Transmit<I, E> for VoidTransmitter
//! where
//!     I: Send,
//! {
//!     async fn transmit(&mut self, item: I) -> Result<(), E>
//!     where
//!         I: 'async_trait,
//!     {
//!         // Do Nothing
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## With async-std/async-channel
//!
//! If you'd like to play with [`async_std::channel::Sender`][] or [`async_channel::Sender`][],
//! use `with-async-channel` feature like:
//!
//! [`async_std::channel::Sender`]: https://docs.rs/async-std/1.9.0/async_std/channel/struct.Sender.html
//! [`async_channel::Sender`]: https://docs.rs/async-channel/1.6.1/async_channel/struct.Sender.html
//!
//! ```toml
//! [dependencies.async-transmit]
//! version = "0.1.0"
//! features = ["with-async-channel"]
//! ```
//!
//! Then you can use `transmit()` method through `Transmit` trait on the sender like:
//!
//! ```
//! # use anyhow::Result;
//! # use futures::executor;
//! # fn main() -> Result<()> {
//! # executor::block_on(async {
//! use async_transmit::*;
//!
//! let (mut s, r) = async_channel::unbounded::<&'static str>();
//!
//! s.transmit("Hello").await?;
//! s.transmit("World").await?;
//! drop(s);
//! assert_eq!(Some("Hello"), r.recv().await.ok());
//! assert_eq!(Some("World"), r.recv().await.ok());
//! assert_eq!(None, r.recv().await.ok());
//! # Ok(())
//! # })
//! # }
//! ```
//!
//! ## With tokio
//!
//! If you'd like to play with [`tokio::sync::mpsc::Sender`][] or [`tokio::sync::mpsc::UnboundedSender`],
//! use `with-tokio` feature like:
//!
//! [`tokio::sync::mpsc::Sender`]: https://docs.rs/tokio/1.3.0/tokio/sync/mpsc/struct.Sender.html
//! [`tokio::sync::mpsc::UnboundedSender`]: https://docs.rs/tokio/1.3.0/tokio/sync/mpsc/struct.UnboundedSender.html
//!
//! ```toml
//! [dependencies.async-transmit]
//! version = "0.1.0"
//! features = ["with-tokio"]
//! ```
//!
//! Then you can use `transmit()` method through `Transmit` trait on the sender like:
//!
//! ```
//! # use anyhow::Result;
//! # use futures::executor;
//! # fn main() -> Result<()> {
//! # executor::block_on(async {
//! use async_transmit::*;
//!
//! let (mut s, mut r) = tokio::sync::mpsc::unbounded_channel::<&'static str>();
//!
//! s.transmit("Hello").await?;
//! s.transmit("World").await?;
//! drop(s);
//! assert_eq!(Some("Hello"), r.recv().await);
//! assert_eq!(Some("World"), r.recv().await);
//! assert_eq!(None, r.recv().await);
//! # Ok(())
//! # })
//! # }
//! ```
//!
//! ## With futures-rs
//!
//! If you'd like to play with [`futures::sink::Sink`], use `with-sink` feature like:
//!
//! [`futures::sink::Sink`]: https://docs.rs/futures/0.3.13/futures/sink/trait.Sink.html
//!
//! ```toml
//! [dependencies.async-transmit]
//! version = "0.1.0"
//! features = ["with-sink"]
//! ```
//!
//! Then you can use `Transmit::from_sink()` to create a wrapper object which implements `Transmit`
//! trait like:
//!
//! ```
//! # use anyhow::Result;
//! # use futures::executor;
//! # fn main() -> Result<()> {
//! # executor::block_on(async {
//! use async_transmit::*;
//! use futures::prelude::*;
//!
//! let (s, mut r) = futures::channel::mpsc::unbounded::<&'static str>();
//! let mut s = Transmit::from_sink(s);
//!
//! s.transmit("Hello").await?;
//! s.transmit("World").await?;
//! drop(s);
//! assert_eq!(Some("Hello"), r.next().await);
//! assert_eq!(Some("World"), r.next().await);
//! assert_eq!(None, r.next().await);
//! # Ok(())
//! # })
//! # }
//! ```
//!
#![doc(test(attr(deny(rust_2018_idioms, warnings))))]
#![doc(test(attr(allow(unused_extern_crates, unused_variables))))]
#![recursion_limit = "2048"]

mod transmit;
pub use transmit::*;
