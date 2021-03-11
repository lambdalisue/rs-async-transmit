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
pub trait Transmit {
    type Item;
    type Error;

    /// Attempts to transmit a value to the peer asynchronously.
    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error>
    where
        Self::Item: 'async_trait;
}

#[async_trait]
impl<T> Transmit for &mut T
where
    T: Transmit + Send + ?Sized,
    T::Item: Send,
{
    type Item = T::Item;
    type Error = T::Error;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error>
    where
        Self::Item: 'async_trait,
    {
        let this = &mut **self;
        this.transmit(item).await
    }
}

/// A helper function to make sure the value is 'Transmit'
#[allow(dead_code)]
pub(crate) fn assert_transmit<I, E, T>(t: T) -> T
where
    T: Transmit<Item = I, Error = E>,
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
pub use from_sink::FromSink;

mod with;
pub use with::With;

impl<T: ?Sized> TransmitExt for T where T: Transmit {}

/// Create `FromSink` object which implements `Transmit` trait from an object which implements
/// `futures::sink::Sink`.
#[cfg(feature = "with-sink")]
pub fn from_sink<S, I>(sink: S) -> FromSink<S, I, S::Error>
where
    S: futures_sink::Sink<I> + Unpin + Send,
    I: Send,
    S::Error: Send,
{
    assert_transmit::<I, S::Error, _>(from_sink::FromSink::from(sink))
}

/// An extension trait for `Transmit`s that provides a variety of convenient
/// functions.
pub trait TransmitExt: Transmit {
    fn with<F, U>(self, f: F) -> with::With<Self, F, Self::Item, U, Self::Error>
    where
        Self: Sized + Send,
        Self::Item: Send,
        Self::Error: Send,
        F: FnMut(U) -> Self::Item + Send,
        U: Send,
    {
        assert_transmit::<U, Self::Error, _>(with::With::new(self, f))
    }
}

#[cfg(test)]
mod tests {
    use super::assert_transmit;
    use super::*;

    use anyhow::Result;
    use futures::channel::mpsc;
    use futures::prelude::*;
    use futures_await_test::async_test;

    #[test]
    fn transmit_mut_ref_ok() -> Result<()> {
        struct DummyTransmitter {}

        #[async_trait]
        impl Transmit for DummyTransmitter {
            type Item = String;
            type Error = anyhow::Error;

            async fn transmit(&mut self, _item: Self::Item) -> Result<(), Self::Error>
            where
                Self::Item: 'async_trait,
            {
                unimplemented!();
            }
        }

        let mut t = DummyTransmitter {};
        let mut_t = &mut t;

        assert_transmit(mut_t);

        Ok(())
    }

    #[cfg(feature = "with-sink")]
    #[async_test]
    async fn transmit_ext_from_sink_is_transmit() -> Result<()> {
        let (s, mut r) = mpsc::unbounded::<&'static str>();

        let mut t = assert_transmit(from_sink(s));
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

        struct DummyTransmitter<T> {
            sender: mpsc::UnboundedSender<T>,
        }

        #[async_trait]
        impl<T> Transmit for DummyTransmitter<T>
        where
            T: Send,
        {
            type Item = T;
            type Error = anyhow::Error;

            async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error>
            where
                Self::Item: 'async_trait,
            {
                self.sender.send(item).await.map_err(Into::into)
            }
        }

        let t = assert_transmit(DummyTransmitter { sender: s });
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
