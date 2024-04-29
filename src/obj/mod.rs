pub mod face;
pub use face::*;

pub mod mesh;
pub use mesh::*;

use nerd::vector::{Vector2, Vector3};
use std::{fs::File, io::Read, path::Path, str::SplitAsciiWhitespace};

use SplitAsciiWhitespace as Tokens;

#[derive(Debug)]
pub enum Error {
    InvalidPath,
    InvalidData,
    UnrecognisedToken(String),
}

pub struct Obj {}

impl Obj {
    /// Wrapper for File::open
    fn open_file<P>(path: P) -> Result<File, Error>
    where
        P: AsRef<Path>,
    {
        match File::open(&path) {
            Ok(file) => Ok(file),
            Err(_) => Err(Error::InvalidPath),
        }
    }

    /// Wrapper for File::read_to_string
    fn read_file(file: &mut File) -> Result<String, Error> {
        let mut t = String::new();
        match file.read_to_string(&mut t) {
            Ok(_) => Ok(t),
            Err(_) => Err(Error::InvalidData),
        }
    }

    /// Utility function for parsing an element of a `Tokens`
    /// iter into a `f32`.
    /// Returns Ok(f32) after successfully parsing, or
    /// Err(Error::InvalidData) if `tokens` doesn't contain
    /// appropriately structured data
    fn parse_float(tokens: &mut Tokens) -> Result<f32, Error> {
        tokens
            .next()
            .ok_or(Error::InvalidData)?
            .parse::<f32>()
            .or(Err(Error::InvalidData))
    }

    /**
    `Obj::import`
    ---
    Import the `.obj` file located at `path`.
    Returns the`Obj` struct generated from
    said file, wrapped in a `Result`.
    */
    pub fn import<P>(path: P) -> Result<ObjMesh, Error>
    where
        P: AsRef<Path>,
    {
        let mut file = Obj::open_file(path)?;
        let text = Obj::read_file(&mut file)?;

        let mut mesh = ObjMesh::empty();

        for line in text.lines() {
            let mut tokens = line.split_ascii_whitespace();
            let command_token = tokens.next().ok_or(Error::InvalidData)?;

            match command_token {
                "mtllib" => {
                    // material library
                    // TODO: return `Obj` instead
                }
                "usemtl" => {
                    // use material
                }
                "o" => {
                    // object name
                    mesh.set_name(tokens.next().or(Some("")).unwrap());
                }
                "v" => {
                    // vertex definition
                    let x = Obj::parse_float(&mut tokens)?;
                    let y = Obj::parse_float(&mut tokens)?;
                    let z = Obj::parse_float(&mut tokens)?;

                    let vertex = Vector3::new(x, y, z);
                    mesh.load_vertex(vertex);
                }
                "vn" => {
                    // vertex normal
                    let x = Obj::parse_float(&mut tokens)?;
                    let y = Obj::parse_float(&mut tokens)?;
                    let z = Obj::parse_float(&mut tokens)?;

                    let normal = Vector3::new(x, y, z);
                    mesh.load_normal(normal);
                }
                "vt" => {
                    // vertex texture (UV)
                    let x = Obj::parse_float(&mut tokens)?;
                    let y = Obj::parse_float(&mut tokens)?;

                    let uv = Vector2::new(x, y);
                    mesh.load_uv(uv);
                }
                "s" => {
                    // surface roughness
                }
                "#" => {
                    // comment
                }
                "f" => {
                    // face data
                    let face_elements = tokens
                        .into_iter()
                        .map(|token| FaceElement::parse(token))
                        .collect();
                    let face: Face = Face::new(face_elements);
                    mesh.load_face(face);
                }
                _ => return Err(Error::UnrecognisedToken(command_token.to_owned())),
            }
        }

        Ok(mesh)
    }
}
