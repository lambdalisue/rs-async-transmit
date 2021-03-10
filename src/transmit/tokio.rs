use async_trait::async_trait;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Sender, UnboundedSender};

use crate::transmit::Transmit;

#[async_trait]
impl<I> Transmit for Sender<I>
where
    I: Send,
{
    type Item = I;
    type Error = SendError<I>;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        Sender::send(self, item).await?;
        Ok(())
    }
}

#[async_trait]
impl<I> Transmit for UnboundedSender<I>
where
    I: Send,
{
    type Item = I;
    type Error = SendError<I>;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        UnboundedSender::send(self, item)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::assert_transmit;
    use super::*;

    use anyhow::Result;
    use futures_await_test::async_test;

    #[async_test]
    async fn tokio_sender_is_transmit() -> Result<()> {
        let (s, mut r) = tokio::sync::mpsc::channel::<&'static str>(10);

        let mut t = assert_transmit(s);
        assert_eq!((), t.transmit("Hello").await?);
        assert_eq!((), t.transmit("World").await?);
        drop(t);
        assert_eq!(r.recv().await, Some("Hello"));
        assert_eq!(r.recv().await, Some("World"));
        assert_eq!(r.recv().await, None);

        Ok(())
    }

    #[async_test]
    async fn tokio_unbounded_sender_is_transmit() -> Result<()> {
        let (s, mut r) = tokio::sync::mpsc::unbounded_channel::<&'static str>();

        let mut t = assert_transmit(s);
        assert_eq!((), t.transmit("Hello").await?);
        assert_eq!((), t.transmit("World").await?);
        drop(t);
        assert_eq!(r.recv().await, Some("Hello"));
        assert_eq!(r.recv().await, Some("World"));
        assert_eq!(r.recv().await, None);

        Ok(())
    }
}
