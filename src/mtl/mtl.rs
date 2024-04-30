use crate::Path;
use crate::{Fs, ImportError, MtlMaterial, Resource};

#[derive(Debug)]
pub struct Mtl {
    materials: Vec<MtlMaterial>,
}

impl Mtl {
    pub fn empty() -> Self {
        Self {
            materials: vec![],
        }
    }
    
    pub fn get_material(&self, material_name: &str) -> Option<&MtlMaterial> {
        for mat in &self.materials {
            if mat.name() == material_name {
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

        Fs::parse_lines(text, |tokens, cmd| {
            match cmd {
                "newmtl" => {
                    let name = tokens.remainder().ok_or(ImportError::InvalidData)?;
                    let mut new_mat = MtlMaterial::empty();
                    new_mat.set_name(name);
                    mats.push(new_mat);
                }
                "map_Kd" => {
                    let map_path =
                        Path::new(tokens.remainder().ok_or(ImportError::InvalidData)?).to_owned();
                    mats.last_mut()
                        .ok_or(ImportError::InvalidData)?
                        .set_diffuse_path(map_path);
                }
                "illum" => {}
                "Ka" => {}
                "Ks" => {}
                "Ns" => {}
                "Ke" => {}
                "Ni" => {}
                "d" => {}
                "#" => {}
                "" => (),
                _ => return Err(ImportError::UnrecognisedToken(cmd.to_owned())),
            }
            Ok(())
        })?;

        Ok(Self { materials: mats })
    }
}
