use async_trait::async_trait;
use futures_sink::Sink;
use futures_util::sink::SinkExt;
use std::marker::PhantomData;

use crate::transmit::Transmit;

#[derive(Debug)]
pub struct FromSink<S, I> {
    sink: S,
    phantom: PhantomData<I>,
}

impl<S, I> FromSink<S, I> {
    fn new(sink: S) -> Self {
        Self {
            sink,
            phantom: PhantomData,
        }
    }

    /// Consumes this transmit, returning the underlying sink.
    pub fn into_inner(self) -> S {
        self.sink
    }

    /// Acquires a reference to the underlying sink that this
    /// transmit is pulling from.
    pub fn get_ref(&self) -> &S {
        &self.sink
    }

    /// Acquires a mutable reference to the underlying sink that
    /// this transmit is pulling from.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.sink
    }
}

#[async_trait]
impl<S, I> Transmit for FromSink<S, I>
where
    I: Send,
    S: Sink<I> + Unpin + Send,
    S::Error: Send,
{
    type Item = I;
    type Error = S::Error;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        SinkExt::send(&mut self.sink, item).await?;
        Ok(())
    }
}

impl<S, I> From<S> for FromSink<S, I>
where
    S: Sink<I>,
{
    fn from(sink: S) -> Self {
        Self::new(sink)
    }
}

#[cfg(test)]
mod tests {
    use super::super::assert_transmit;
    use super::*;

    use anyhow::Result;
    use futures::channel::mpsc;
    use futures::prelude::*;
    use futures_await_test::async_test;

    #[async_test]
    async fn from_sink_is_transmit() -> Result<()> {
        let (s, mut r) = mpsc::unbounded::<&'static str>();

        let mut t = assert_transmit(FromSink::from(s));
        t.transmit("Hello").await?;
        t.transmit("World").await?;
        drop(t);
        assert_eq!(r.next().await, Some("Hello"));
        assert_eq!(r.next().await, Some("World"));
        assert_eq!(r.next().await, None);

        Ok(())
    }
}
