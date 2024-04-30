/**
crate::ImportError
---
Custom error type meant to be returned as
`Err(Error)` variant of `Result` type inside
crate specific falible function calls
*/
#[derive(Debug)]
pub enum ImportError {
    InvalidPath,
    InvalidData,
    UnrecognisedToken(String),
}
