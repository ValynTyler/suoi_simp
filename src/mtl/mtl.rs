use nerd::vector::Vector3;

use crate::Path;
use crate::{Fs, ImportError, MtlMaterial, Resource};

#[derive(Debug)]
pub struct Mtl {
    materials: Vec<MtlMaterial>,
}

impl Mtl {
    pub fn empty() -> Self {
        Self { materials: vec![] }
    }

    pub fn get_material(&self, material_name: &str) -> Option<&MtlMaterial> {
        for mat in &self.materials {
            if mat.get_name() == material_name {
                return Some(&mat);
            }
        }

        None
    }
}

impl Resource for Mtl {
    /**
    `Mtl::import`
    ---
    Import the `.mtl` file located at `path`.
    Returns the `Mtl` struct generated from
    said file, wrapped in a `Result`.
    */
    fn import(path: &Path) -> Result<Self, ImportError> {
        let mut file = Fs::open_file(&path)?;
        let text = Fs::read_file(&mut file)?;

        let mut mats: Vec<MtlMaterial> = vec![];

        Fs::parse_lines(text, |mut tokens, cmd| {
            match cmd {
                "newmtl" => {
                    // define new material
                    let name = tokens.remainder().ok_or(ImportError::InvalidData)?;
                    let mut new_mat = MtlMaterial::empty();
                    new_mat.set_name(name);
                    mats.push(new_mat);
                }
                "map_Kd" => {
                    // define diffuse texmap path
                    let map_path =
                        Path::new(tokens.remainder().ok_or(ImportError::InvalidData)?).to_owned();
                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_diffuse_path(map_path);
                }
                "illum" => {
                    // define illumination model
                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_illum(Fs::parse_uint(&mut tokens)?);
                }
                "Ka" => {
                    // define ambient color
                    let col = Vector3::new(
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                    );

                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_ambient_color(col);
                }
                "Kd" => {
                    // define diffuse color
                    let col = Vector3::new(
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                    );

                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_diffuse_color(col);
                }
                "Ks" => {
                    // define specular color
                    let col = Vector3::new(
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                    );

                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_specular_color(col);
                }
                "Ns" => {
                    // define specular exponent
                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_specular_exponent(Fs::parse_float(&mut tokens)?);
                }
                "Ke" => {
                    // define emissive
                    let col = Vector3::new(
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                        Fs::parse_float(&mut tokens)?,
                    );

                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_emissive(col);
                }
                "Ni" => {
                    // define optical density
                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_optical_density(Fs::parse_float(&mut tokens)?);
                }
                "d" => {
                    // define transparency
                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_opacity(Fs::parse_float(&mut tokens)?);
                }
                "#" => {
                    // comment
                }
                "" => (),
                _ => return Err(ImportError::UnrecognisedToken(cmd.to_owned())),
            }
            Ok(())
        })?;

        Ok(Self { materials: mats })
    }
}
