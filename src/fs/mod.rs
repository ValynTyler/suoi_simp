use std::{fs::File, io::Read};

use crate::ImportError;

pub type Path = std::path::Path;
pub type PathBuf = std::path::PathBuf;
pub type Tokens<'a> = std::str::SplitAsciiWhitespace<'a>;

pub struct Fs;

impl Fs {
    /**
    Fs::open_file
    ---
    Wrapper for File::open
    */
    pub fn open_file(path: &Path) -> Result<File, ImportError> {
        match File::open(&path) {
            Ok(file) => Ok(file),
            Err(_) => Err(ImportError::InvalidPath(path.to_owned())),
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
    Fs::read_bytes
    ---
    Wrapper for File::read
    */
    pub fn read_bytes(file: &mut File) -> Result<Vec<u8>, ImportError> {
        let mut t: Vec<u8> = vec![];
        match file.read_to_end(&mut t) {
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

    pub fn parse_uint(tokens: &mut Tokens) -> Result<u32, ImportError> {
        tokens
            .next()
            .ok_or(ImportError::InvalidData)?
            .parse::<u32>()
            .or(Err(ImportError::InvalidData))
    }

    pub fn parse_lines<F>(text: String, mut f: F) -> Result<(), ImportError>
    where
        F: FnMut(Tokens, &str) -> Result<(), ImportError>,
    {
        for line in text.lines() {
            let mut tokens = line.split_ascii_whitespace();
            let cmd_token = tokens.next().or(Some("")).unwrap();

            (f)(tokens, cmd_token)?;
        }

        Ok(())
    }
}
