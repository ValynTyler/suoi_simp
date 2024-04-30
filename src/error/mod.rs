use std::path::PathBuf;

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
