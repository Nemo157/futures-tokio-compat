use crate::{Compat, LocalFrom};
use core::{future::Future, pin::Pin};

impl<T> tokio_executor::Executor for Compat<T>
where
    T: futures_core::task::Spawn,
{
    fn spawn(
        &mut self,
        future: Pin<Box<dyn Future<Output = ()> + Send>>,
    ) -> Result<(), tokio_executor::SpawnError> {
        self.inner.spawn_obj(future.into()).map_err(LocalFrom::from)
    }

    fn status(&self) -> Result<(), tokio_executor::SpawnError> {
        self.inner.status().map_err(LocalFrom::from)
    }
}

impl<T, Fut> tokio_executor::TypedExecutor<Fut> for Compat<T>
where
    T: futures_core::task::Spawn,
    Fut: core::future::Future<Output = ()> + Send + 'static,
{
    fn spawn(&mut self, future: Fut) -> Result<(), tokio_executor::SpawnError> {
        self.inner
            .spawn_obj(Box::pin(future).into())
            .map_err(LocalFrom::from)
    }

    fn status(&self) -> Result<(), tokio_executor::SpawnError> {
        self.inner.status().map_err(LocalFrom::from)
    }
}

impl<T> futures_core::task::Spawn for Compat<T>
where
    T: tokio_executor::Executor,
{
    fn spawn_obj(
        &mut self,
        future: futures_core::future::FutureObj<'static, ()>,
    ) -> Result<(), futures_core::task::SpawnError> {
        self.inner.spawn(Box::pin(future)).map_err(LocalFrom::from)
    }

    fn status(&self) -> Result<(), futures_core::task::SpawnError> {
        self.inner.status().map_err(LocalFrom::from)
    }
}
