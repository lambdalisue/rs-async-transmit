use async_trait::async_trait;
use core::marker::PhantomData;

use crate::transmit::Transmit;

pub struct With<T, F, I, U, E> {
    inner: T,
    f: F,
    phantom: PhantomData<(I, U, E)>
}

impl<T, F, I, U, E> With<T, F, I, U, E>
where
    T: Transmit<I, E> + Send,
    I: Send,
    E: Send,
{
    pub(crate) fn new(inner: T, f: F) -> Self {
        Self {
            inner,
            f,
            phantom: PhantomData,
        }
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
}

#[async_trait]
impl<T, F, I, U, E> Transmit<U, E> for With<T, F, I, U, E>
where
    T: Transmit<I, E> + Send,
    F: FnMut(U) -> I + Send,
    I: Send,
    U: Send,
    E: Send,
{
    async fn transmit(&mut self, item: U) -> Result<(), E> {
        let item = (self.f)(item);
        self.inner.transmit(item).await
    }
}

