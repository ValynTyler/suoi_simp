use std::{fs::File, io::Read};

use crate::ImportError;

pub use std::path::Path;
pub use std::str::SplitAsciiWhitespace as Tokens;

pub struct Fs;

impl Fs {
    /**
    Fs::open_file
    ---
    Wrapper for File::open
    */
    pub fn open_file<P>(path: P) -> Result<File, ImportError>
    where
        P: AsRef<Path>,
    {
        match File::open(&path) {
            Ok(file) => Ok(file),
            Err(_) => Err(ImportError::InvalidPath),
        }
    }

    /**
    Fs::read_file
    ---
    Wrapper for File::read_to_string
    */
    pub fn read_file(file: &mut File) -> Result<String, ImportError> {
        let mut t = String::new();
        match file.read_to_string(&mut t) {
            Ok(_) => Ok(t),
            Err(_) => Err(ImportError::InvalidData),
        }
    }

    /**
    Fs::parse_float
    ---
    Consumes the next element of `tokens` and returns `Ok(f32) if the
    parse is successful, or Err(ImportError:InvalidData) if the consumed
    data doesn't contain a float
    */
    pub fn parse_float(tokens: &mut Tokens) -> Result<f32, ImportError> {
        tokens
            .next()
            .ok_or(ImportError::InvalidData)?
            .parse::<f32>()
            .or(Err(ImportError::InvalidData))
    }
}
