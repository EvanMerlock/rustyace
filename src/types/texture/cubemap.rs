use crate::gl;
use crate::types::*;
use std::rc::Rc;
use std::path::{Path, PathBuf};
use std::fs;

pub struct CubemapPaths {
    top: PathBuf,      // Pos Y
    bottom: PathBuf,   // Neg Y
    left: PathBuf,     // Neg X
    right: PathBuf,    // Pos X
    front: PathBuf,    // Neg Z
    back: PathBuf,     // Pos Z
}

impl CubemapPaths {

    pub fn from_directory<T: AsRef<Path>>(dir_path: T) -> Result<CubemapPaths, ::std::io::Error> {
        let path = dir_path.as_ref();

        let mut top = path.to_path_buf();
        top.push("top.jpg");
        let _ = fs::metadata(&top)?;

        let mut bottom = path.to_path_buf();
        bottom.push("bottom.jpg");
        let _ = fs::metadata(&bottom)?;

        let mut left = path.to_path_buf();
        left.push("left.jpg");
        let _ = fs::metadata(&left)?;

        let mut right = path.to_path_buf();
        right.push("right.jpg");
        let _ = fs::metadata(&right)?;

        let mut front = path.to_path_buf();
        front.push("front.jpg");
        let _ = fs::metadata(&front)?;

        let mut back = path.to_path_buf();
        back.push("back.jpg");
        let _ = fs::metadata(&back)?;

        Ok(CubemapPaths::from_raw_paths(top, bottom, left, right, front, back))
    }

    fn from_raw_paths(top: PathBuf, bottom: PathBuf, left: PathBuf, right: PathBuf, front: PathBuf, back: PathBuf) -> CubemapPaths {
        CubemapPaths {
            top,
            bottom,
            left,
            right,
            front,
            back
        }
    }

    pub(crate) fn cubemap_entries(self) -> Vec<(PathBuf, CubemapTextureType)> {
        let mut cm_e = Vec::new();

        cm_e.push((self.right, CubemapTextureType::TextureCubeMapPosX));
        cm_e.push((self.left, CubemapTextureType::TextureCubeMapNegX));

        cm_e.push((self.top, CubemapTextureType::TextureCubeMapPosY));
        cm_e.push((self.bottom, CubemapTextureType::TextureCubeMapNegY));

        cm_e.push((self.front, CubemapTextureType::TextureCubeMapPosZ));
        cm_e.push((self.back, CubemapTextureType::TextureCubeMapNegZ));

        cm_e
    }
}

pub enum CubemapTextureType {
    TextureCubeMapPosX      = gl::TEXTURE_CUBE_MAP_POSITIVE_X as isize,
    TextureCubeMapNegX      = gl::TEXTURE_CUBE_MAP_NEGATIVE_X as isize,

    TextureCubeMapPosY      = gl::TEXTURE_CUBE_MAP_POSITIVE_Y as isize,
    TextureCubeMapNegY      = gl::TEXTURE_CUBE_MAP_NEGATIVE_Y as isize,

    TextureCubeMapPosZ      = gl::TEXTURE_CUBE_MAP_POSITIVE_Z as isize,
    TextureCubeMapNegZ      = gl::TEXTURE_CUBE_MAP_NEGATIVE_Z as isize,
}