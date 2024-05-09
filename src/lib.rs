#![feature(str_split_whitespace_remainder)]

pub mod obj;
pub use obj::*;

pub mod mtl;
pub use mtl::*;

pub mod fs;
pub use fs::*;

/**
crate::ImportError
---
Custom error type meant to be returned as
`Err(Error)` variant of `Result` type inside
crate specific falible function calls
*/
#[derive(Debug)]
pub enum ImportError {
    InvalidData,
    InvalidPath(PathBuf),
    UnrecognisedToken(String),
}

pub trait Resource {
    fn import(path: &Path) -> Result<Self, ImportError>
    where
        Self: Sized;
}
