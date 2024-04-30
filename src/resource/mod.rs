use crate::{ImportError, Path};

pub trait Resource {
    fn import<P>(path: P) -> Result<Self, ImportError>
    where
        P: AsRef<Path>,
        Self: Sized;
}
