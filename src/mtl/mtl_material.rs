use nerd::vector::Vector3 as Color;

use crate::PathBuf;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct MtlMaterial {
    name: String,

    // tokens: Ka, Kd
    ambient_color: Color,
    diffuse_color: Color,

    // tokens: Ks, Ns
    specular_color: Color,
    specular_exponent: f32,

    // token: Ke
    emissive: Color,

    // token: d
    opacity: f32,

    // token: Ni
    optical_density: f32,

    // token: illum
    illumination_model: u32,

    // tokens: mapd_Ka, map_Kd, map_d
    ambient_texmap: Option<PathBuf>,
    diffuse_texmap: Option<PathBuf>,
    alpha_texmap: Option<PathBuf>,
}

impl MtlMaterial {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.to_string();
    }

    pub fn get_diffuse_path(&self) -> &Option<PathBuf> {
        &self.diffuse_texmap
    }

    pub fn set_diffuse_path(&mut self, path: PathBuf) {
        self.diffuse_texmap = Some(path);
    }

    pub fn empty() -> Self {
        Self {
            name: String::new(),
            ambient_color: Color::ZERO,
            diffuse_color: Color::ZERO,
            specular_color: Color::ZERO,
            specular_exponent: 0.0,
            emissive: Color::ZERO,
            opacity: 1.0,
            optical_density: 1.0,
            illumination_model: 0,
            ambient_texmap: None,
            diffuse_texmap: None,
            alpha_texmap: None,
        }
    }
}
