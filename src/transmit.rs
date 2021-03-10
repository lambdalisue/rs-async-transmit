use async_trait::async_trait;

/// The `Transmit` trait allows for transmitting item to a peer.
///
/// Implementors of the `Transmit` trait are called 'transmitters'.
///
/// Transmitters are defined by one required method, [`transmit()`].
/// The method will attempt to transmit some data to a peer asynchronously, returning
/// if the transmission has succeeded.
#[must_use = "transmit do nothing unless polled"]
#[async_trait]
pub trait Transmit<I, E>
{
    /// Attempts to transmit a value to the peer asynchronously.
    async fn transmit(&mut self, item: I) -> Result<(), E>
    where
        I: 'async_trait;
}

/// A helper function to make sure the value is 'Transmit'
#[allow(dead_code)]
pub(crate) fn assert_transmit<I, E, T>(t: T) -> T
where
    T: Transmit<I, E>,
{
    t
}

#[cfg(feature = "with-async-channel")]
mod async_channel;
#[cfg(feature = "with-async-channel")]
pub use self::async_channel::*;

#[cfg(feature = "with-tokio")]
mod tokio;
#[cfg(feature = "with-tokio")]
pub use self::tokio::*;

#[cfg(feature = "with-sink")]
mod from_sink;
#[cfg(feature = "with-sink")]
pub use from_sink::*;

mod with;
pub use with::With;

impl<T: ?Sized, I, E> TransmitExt<I, E> for T 
where 
    T: Transmit<I, E>,
    I: Send,
    E: Send,
{}

/// An extension trait for `Transmit`s that provides a variety of convenient
/// functions.
pub trait TransmitExt<I, E>: Transmit<I, E>
{
    #[cfg(feature = "with-sink")]
    /// Create `FromSink` object which implements `Transmit` trait from an object which implements
    /// `futures::sink::Sink`.
    fn from_sink<S>(sink: S) -> from_sink::FromSink<S, I, E>
    where
        I: Send,
        E: Send,
        S: futures_sink::Sink<I, Error = E> + Unpin + Send,
    {
        assert_transmit::<I, E, _>(from_sink::FromSink::from(sink))
    }

    fn with<F, U>(self, f: F) -> with::With<Self, F, I, U, E>
    where
        Self: Sized + Send,
        I: Send,
        E: Send,
        F: FnMut(U) -> I + Send,
        U: Send,
    {
        assert_transmit::<U, E, _>(with::With::new(self, f))
    }
}

#[cfg(test)]
#[cfg(feature = "with-sink")]
mod sink_tests {
    use super::assert_transmit;
    use super::*;

    use anyhow::Result;
    use futures::channel::mpsc;
    use futures::prelude::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn transmit_ext_from_sink_is_transmit() -> Result<()> {
        let (s, mut r) = mpsc::unbounded::<&'static str>();

        let mut t = assert_transmit(Transmit::from_sink(s));
        assert_eq!((), t.transmit("Hello").await?);
        assert_eq!((), t.transmit("World").await?);
        drop(t);
        assert_eq!(r.next().await, Some("Hello"));
        assert_eq!(r.next().await, Some("World"));
        assert_eq!(r.next().await, None);

        Ok(())
    }

    #[async_test]
    async fn transmit_ext_with_is_transmit() -> Result<()> {
        let (s, mut r) = mpsc::unbounded::<String>();

        let t = assert_transmit(Transmit::from_sink(s));
        let mut t = t.with(|s| format!("!!!{}!!!", s));
        assert_eq!((), t.transmit("Hello").await?);
        assert_eq!((), t.transmit("World").await?);
        drop(t);
        assert_eq!(r.next().await, Some("!!!Hello!!!".to_string()));
        assert_eq!(r.next().await, Some("!!!World!!!".to_string()));
        assert_eq!(r.next().await, None);

        Ok(())
    }
}
