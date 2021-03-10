use async_channel::{SendError, Sender};
use async_trait::async_trait;

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

#[cfg(test)]
mod tests {
    use super::super::assert_transmit;
    use super::*;

    use anyhow::Result;
    use futures_await_test::async_test;

    #[async_test]
    async fn async_channel_sender_is_transmit() -> Result<()> {
        let (s, r) = async_channel::unbounded::<&'static str>();

        let mut t = assert_transmit(s);
        assert_eq!((), t.transmit("Hello").await?);
        assert_eq!((), t.transmit("World").await?);
        drop(t);
        assert_eq!(r.recv().await.ok(), Some("Hello"));
        assert_eq!(r.recv().await.ok(), Some("World"));
        assert_eq!(r.recv().await.ok(), None);

        Ok(())
    }
}
