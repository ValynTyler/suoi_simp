use nerd::vector::{Vector2, Vector3};

use crate::face::Face;
use crate::face::FaceElement;
use crate::fs::Path;
use crate::obj_mesh::ObjMesh;
use crate::Fs;
use crate::ImportError;
use crate::Mtl;
use crate::Resource;

#[derive(Debug)]
pub struct Obj {
    meshes: Vec<ObjMesh>,
}

impl Obj {
    pub fn meshes(&self) -> &Vec<ObjMesh> {
        &self.meshes
    }

    pub fn all_pos_data(&self) -> Vec<Vector3> {
        let mut data = vec![];

        for mesh in &self.meshes {
            for pos in mesh.positions() {
                data.push(*pos)
            }
        }

        data
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
    fn import(path: &Path) -> Result<Self, ImportError> {
        let mut file = Fs::open_file(path)?;
        let text = Fs::read_file(&mut file)?;

        let mut meshes: Vec<ObjMesh> = vec![];
        let mut mtl = Mtl::empty();

        Fs::parse_lines(text, |mut tokens, cmd| {
            match cmd {
                "mtllib" => {
                    // material library
                    let mtl_file = tokens.remainder().ok_or(ImportError::InvalidData)?;
                    let mut mtl_path = path
                        .parent()
                        .ok_or(ImportError::InvalidPath(path.to_owned()))?
                        .to_owned();
                    mtl_path.push(mtl_file);

                    mtl = Mtl::import(&mtl_path)?;
                }
                "usemtl" => {
                    // use material
                    let mat_name = tokens.remainder().ok_or(ImportError::InvalidData)?;
                    let new_mat = mtl.get_material(mat_name).ok_or(ImportError::InvalidData)?;
                    
                    meshes
                        .last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_material(new_mat.clone());
                }
                "o" => {
                    // object name
                    let mut new_mesh = ObjMesh::empty();
                    new_mesh.set_name(tokens.remainder().ok_or(ImportError::InvalidData)?);

                    meshes.push(new_mesh);
                }
                "v" => {
                    // vertex definition
                    let x = Fs::parse_float(&mut tokens)?;
                    let y = Fs::parse_float(&mut tokens)?;
                    let z = Fs::parse_float(&mut tokens)?;

                    let vertex = Vector3::new(x, y, z);
                    meshes
                        .last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .load_position(vertex);
                }
                "vn" => {
                    // vertex normal
                    let x = Fs::parse_float(&mut tokens)?;
                    let y = Fs::parse_float(&mut tokens)?;
                    let z = Fs::parse_float(&mut tokens)?;

                    let normal = Vector3::new(x, y, z);
                    meshes
                        .last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .load_normal(normal);
                }
                "vt" => {
                    // vertex texture (UV)
                    let x = Fs::parse_float(&mut tokens)?;
                    let y = Fs::parse_float(&mut tokens)?;

                    let uv = Vector2::new(x, y);
                    meshes
                        .last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .load_uv(uv);
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
                    meshes
                        .last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .load_face(face);
                }
                "" => (),
                _ => return Err(ImportError::UnrecognisedToken(cmd.to_owned())),
            }
            Ok(())
        })?;

        Ok(Self { meshes })
    }
}
