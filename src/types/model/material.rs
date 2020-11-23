use std::path::PathBuf;
use std::collections::HashMap;
use tobj;

pub struct Material {
    pub name: String,
    colors: MatColor,
    texture_paths: MatTexturePaths,
    extra_parameters: HashMap<String, String>,
}

struct MatColor {
    ambient: [f32; 3],
    diffuse: [f32; 3],
    specular: [f32; 3],

    shininess: f32,
    dissolve: f32,
    optical_density: f32,
}

struct MatTexturePaths {
    ambient: PathBuf,
    diffuse: PathBuf,
    specular: PathBuf,
    normal: PathBuf,
    shininess: PathBuf,
    dissolve: PathBuf,
}

impl From<tobj::Material> for Material {
    fn from(mat: tobj::Material) -> Self {
        Material {
            name: mat.name,
            colors: MatColor {
                ambient: mat.ambient,
                diffuse: mat.diffuse,
                specular: mat.specular,
                shininess: mat.shininess,
                dissolve: mat.dissolve,
                optical_density: mat.optical_density,
            },
            texture_paths: MatTexturePaths {
                ambient: PathBuf::from(mat.ambient_texture),
                diffuse: PathBuf::from(mat.diffuse_texture),
                specular: PathBuf::from(mat.specular_texture),
                normal: PathBuf::from(mat.normal_texture),
                shininess: PathBuf::from(mat.shininess_texture),
                dissolve: PathBuf::from(mat.dissolve_texture),
            },
            extra_parameters: mat.unknown_param,
        }
    }
}