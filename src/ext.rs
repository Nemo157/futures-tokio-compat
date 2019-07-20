use crate::Compat;

pub trait FuturesSpawnCompatExt: Sized {
    fn compat(self) -> Compat<Self>;
}

pub trait TokioExecutorCompatExt: Sized {
    fn compat(self) -> Compat<Self>;
}

impl<T> FuturesSpawnCompatExt for T
where
    T: futures_core::task::Spawn + Sized,
{
    fn compat(self) -> Compat<Self> {
        Compat { inner: self }
    }
}

impl<T> TokioExecutorCompatExt for T
where
    T: tokio_executor::Executor + Sized,
{
    fn compat(self) -> Compat<Self> {
        Compat { inner: self }
    }
}
