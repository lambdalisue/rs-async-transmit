[![crates.io](https://img.shields.io/crates/v/async-transmit.svg)](https://crates.io/crates/async-transmit)
[![dependency status](https://deps.rs/repo/github/lambdalisue/rs-async-transmit/status.svg)](https://deps.rs/repo/github/lambdalisue/rs-async-transmit)
[![docs.rs](https://docs.rs/async-transmit/badge.svg)](https://docs.rs/async-transmit)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://github.com/lambdalisue/rs-async-transmit/actions/workflows/build.yml/badge.svg)](https://github.com/lambdalisue/rs-async-transmit/actions/workflows/build.yml)
[![Test](https://github.com/lambdalisue/rs-async-transmit/actions/workflows/test.yml/badge.svg)](https://github.com/lambdalisue/rs-async-transmit/actions/workflows/test.yml)
[![Audit](https://github.com/lambdalisue/rs-async-transmit/actions/workflows/audit.yml/badge.svg)](https://github.com/lambdalisue/rs-async-transmit/actions/workflows/audit.yml)
[![codecov](https://codecov.io/github/lambdalisue/rs-async-transmit/branch/main/graph/badge.svg?token=O2TF00WUP7)](https://codecov.io/github/lambdalisue/rs-async-transmit)

# async-transmit

## Async trait for transmitting data to peers

`async-transmit` crate provides `Transmit` trait which allows value to be transmit asynchronously.

This crate relies on [async-trait][], the original definition of the `Transmit` trait is:

[async-trait]: https://github.com/dtolnay/async-trait

```rust
use async_trait::async_trait;

#[async_trait]
pub trait Transmit {
    type Item;
    type Error;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error>
    where
        Self::Item: 'async_trait;
}
```

So use `#[async_trait]` to when implement `Transmit` like:

```rust
use async_transmit::*;
use async_trait::async_trait;

struct VoidTransmitter<I, E> {
    phantom: std::marker::PhantomData<(I, E)>,
}

#[async_trait]
impl<I, E> Transmit for VoidTransmitter<I, E>
where
    I: Send,
    E: Send,
{
    type Item = I;
    type Error = E;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error>
    where
        I: 'async_trait,
    {
        // Do Nothing
        Ok(())
    }
}
```

### With async-std/async-channel

If you'd like to play with [`async_std::channel::Sender`][] or [`async_channel::Sender`][],
use `with-async-channel` feature like:

[`async_std::channel::sender`]: https://docs.rs/async-std/1.9.0/async_std/channel/struct.Sender.html
[`async_channel::sender`]: https://docs.rs/async-channel/1.6.1/async_channel/struct.Sender.html

```toml
[dependencies.async-transmit]
version = "0.1.0"
features = ["with-async-channel"]
```

Then you can use `transmit()` method through `Transmit` trait on the sender like:

```rust
use async_transmit::*;

let (mut s, r) = async_channel::unbounded::<&'static str>();

s.transmit("Hello").await?;
s.transmit("World").await?;
drop(s);
assert_eq!(Some("Hello"), r.recv().await.ok());
assert_eq!(Some("World"), r.recv().await.ok());
assert_eq!(None, r.recv().await.ok());
```

### With tokio

If you'd like to play with [`tokio::sync::mpsc::Sender`][] or [`tokio::sync::mpsc::UnboundedSender`],
use `with-tokio` feature like:

[`tokio::sync::mpsc::sender`]: https://docs.rs/tokio/1.3.0/tokio/sync/mpsc/struct.Sender.html
[`tokio::sync::mpsc::unboundedsender`]: https://docs.rs/tokio/1.3.0/tokio/sync/mpsc/struct.UnboundedSender.html

```toml
[dependencies.async-transmit]
version = "0.1.0"
features = ["with-tokio"]
```

Then you can use `transmit()` method through `Transmit` trait on the sender like:

```rust
use async_transmit::*;

let (mut s, mut r) = tokio::sync::mpsc::unbounded_channel::<&'static str>();

s.transmit("Hello").await?;
s.transmit("World").await?;
drop(s);
assert_eq!(Some("Hello"), r.recv().await);
assert_eq!(Some("World"), r.recv().await);
assert_eq!(None, r.recv().await);
```

### With futures-rs

If you'd like to play with [`futures::sink::Sink`], use `with-sink` feature like:

[`futures::sink::sink`]: https://docs.rs/futures/0.3.13/futures/sink/trait.Sink.html

```toml
[dependencies.async-transmit]
version = "0.1.0"
features = ["with-sink"]
```

Then you can use `async_transmit::from_sink()` to create a wrapper object which implements `Transmit`
trait like:

```rust
use async_transmit::*;
use futures::prelude::*;

let (s, mut r) = futures::channel::mpsc::unbounded::<&'static str>();
let mut s = from_sink(s);

s.transmit("Hello").await?;
s.transmit("World").await?;
drop(s);
assert_eq!(Some("Hello"), r.next().await);
assert_eq!(Some("World"), r.next().await);
assert_eq!(None, r.next().await);
```

# License

The code follows MIT license written in [LICENSE](./LICENSE). Contributors need
to agree that any modifications sent in this repository follow the license.
