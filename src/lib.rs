mod error;
mod ext;
mod spawn;

pub struct Compat<T> {
    inner: T,
}

pub(crate) trait LocalFrom<T> {
    fn from(t: T) -> Self;
}

mod prelude {
    pub use crate::ext::{FuturesSpawnCompatExt as _, TokioExecutorCompatExt as _};
}
