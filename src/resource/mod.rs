use crate::{ImportError, Path};

pub trait Resource {
    fn import(path: &Path) -> Result<Self, ImportError>
        where Self: Sized;
}
