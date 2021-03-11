use async_trait::async_trait;

use crate::transmit::Transmit;

pub struct TransmitMapErr<T, F> {
    inner: T,
    f: Option<F>,
}

impl<T: Unpin, F> Unpin for TransmitMapErr<T, F> {}

impl<T, F> TransmitMapErr<T, F> {
    pub(crate) fn new(inner: T, f: F) -> Self {
        Self { inner, f: Some(f) }
    }

    /// Consumes this combinator, returning the underlying transmit.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Acquires a reference to the underlying transmit that this
    /// combinator is pulling from.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Acquires a mutable reference to the underlying transmit that
    /// this combinator is pulling from.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    fn take_f(&mut self) -> F {
        self.f
            .take()
            .expect("polled TransmitMapErr after completion")
    }
}

#[async_trait]
impl<T, F, E> Transmit for TransmitMapErr<T, F>
where
    T: Transmit + Send,
    T::Item: Send,
    F: FnOnce(T::Error) -> E + Send,
{
    type Item = T::Item;
    type Error = E;

    async fn transmit(&mut self, item: Self::Item) -> Result<(), Self::Error> {
        self.inner
            .transmit(item)
            .await
            .map_err(|e| self.take_f()(e))
    }
}
