use nerd::vector::{Vector2, Vector3};

use crate::face::Face;
use crate::face::FaceElement;
use crate::fs::Path;
use crate::mesh::ObjMesh;
use crate::Fs;
use crate::ImportError;

pub struct Obj {}

impl Obj {
    /**
    `Obj::import`
    ---
    Import the `.obj` file located at `path`.
    Returns the`Obj` struct generated from
    said file, wrapped in a `Result`.
    */
    pub fn import<P>(path: P) -> Result<ObjMesh, ImportError>   // TODO: Create `Resource` trait to contain this and `Mtl`
    where
        P: AsRef<Path>,
    {
        let mut file = Fs::open_file(path)?;
        let text = Fs::read_file(&mut file)?;

        let mut mesh = ObjMesh::empty();

        for line in text.lines() {
            let mut tokens = line.split_ascii_whitespace();
            let command_token = tokens.next().ok_or(ImportError::InvalidData)?;

            match command_token {
                "mtllib" => {
                    // material library
                    
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
                    let x = Fs::parse_float(&mut tokens)?;
                    let y = Fs::parse_float(&mut tokens)?;
                    let z = Fs::parse_float(&mut tokens)?;

                    let vertex = Vector3::new(x, y, z);
                    mesh.load_vertex(vertex);
                }
                "vn" => {
                    // vertex normal
                    let x = Fs::parse_float(&mut tokens)?;
                    let y = Fs::parse_float(&mut tokens)?;
                    let z = Fs::parse_float(&mut tokens)?;

                    let normal = Vector3::new(x, y, z);
                    mesh.load_normal(normal);
                }
                "vt" => {
                    // vertex texture (UV)
                    let x = Fs::parse_float(&mut tokens)?;
                    let y = Fs::parse_float(&mut tokens)?;

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
                _ => return Err(ImportError::UnrecognisedToken(command_token.to_owned())),
            }
        }

        Ok(mesh)
    }
}
