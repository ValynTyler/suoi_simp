use crate::Path;
use crate::{Fs, ImportError, MtlMaterial, Resource};

#[derive(Debug)]
pub struct Mtl {
    material: MtlMaterial,
}

impl Mtl {
    pub fn material(&self) -> &MtlMaterial {
        &self.material
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

        let mut mat = MtlMaterial::empty();

        Fs::parse_lines(text, |tokens, cmd| {
            match cmd {
                "newmtl" => {
                    *mat.name() = tokens
                        .remainder()
                        .ok_or(ImportError::InvalidData)?
                        .to_owned();
                }
                "map_Kd" => {
                    let map_path = Path::new(tokens
                        .remainder()
                        .ok_or(ImportError::InvalidData)?)
                        .to_owned();
                    mat.diffuse_path(map_path);
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

        Ok(Self { material: mat })
    }
}
