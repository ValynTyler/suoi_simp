use nerd::vector::{Vector2, Vector3};

use crate::face::Face;
use crate::face::FaceElement;
use crate::fs::Path;
use crate::mesh::ObjMesh;
use crate::Fs;
use crate::ImportError;
use crate::Resource;

pub struct Obj {
    mesh: ObjMesh,
}

impl Obj {
    pub fn mesh(&self) -> &ObjMesh {
        &self.mesh
    }
}

impl Resource for Obj {
    /**
    `Obj::import`
    ---
    Import the `.obj` file located at `path`.
    Returns the `Obj` struct generated from
    said file, wrapped in a `Result`.
    */
    fn import<P>(path: P) -> Result<Obj, ImportError>
    where
        P: AsRef<Path>,
    {
        let mut file = Fs::open_file(path)?;
        let text = Fs::read_file(&mut file)?;

        let mut mesh = ObjMesh::empty();

        Fs::parse_lines(text, |mut tokens, cmd| {
            match cmd {
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
                "" => (),
                _ => return Err(ImportError::UnrecognisedToken(cmd.to_owned())),
            }
            Ok(())
        })?;

        Ok(Self { mesh })
    }
}
