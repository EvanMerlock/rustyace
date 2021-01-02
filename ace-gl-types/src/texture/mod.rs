use image;
use std::rc::Rc;
use crate::gl;
use std::path::Path;
use thiserror::Error;
use std::ptr;
use crate::types::*;
use std::io;

mod gl_internalstorage;
mod pix_type;
mod texture_configuration;
mod gl_texturetype;
mod cubemap;

pub use self::gl_internalstorage::*;
pub use self::pix_type::*;
pub use self::texture_configuration::*;
pub use self::gl_texturetype::*;
pub use self::cubemap::*;

pub struct Texture {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
    tex_cfg: TexConfig,
}

impl Texture {
    pub fn from_file<P: AsRef<Path>>(gl_ctx: Rc<gl::Gl>, path: P, tex_cfg: TexConfig) -> Result<Texture, TextureError> {
        tex_cfg.validate()?;
        let mut dyn_img = image::open(path)?;
        dyn_img = dyn_img.flipv();
        let rgb_image = dyn_img.to_rgb();
        let mut tex_id: u32 = 0;
        unsafe {
            gl_ctx.GenTextures(1, &mut tex_id);
            gl_ctx.BindTexture(tex_cfg.tex_type as u32, tex_id);
            
            // Load texture into memory.
            // TODO: not all images are RGB. Switch based on the image properties and error out if invalid pix format is used.
            // Right now, we don't intfer anything and have the user specify the data format. Maybe make this safer?
            let width = rgb_image.width() as i32;
            let height = rgb_image.height() as i32;
            let bytes = rgb_image.into_vec();
            gl_ctx.TexImage2D(tex_cfg.tex_type as u32, 0, tex_cfg.internal_fmt as i32, width, height, 0, tex_cfg.pix_data_fmt as u32, tex_cfg.pix_type_fmt as u32, bytes.as_ptr() as *const _);
            gl_ctx.GenerateMipmap(tex_cfg.tex_type as u32);

            // Set texture wrap/filtering settings for _current_ texture
            // TODO: Make this configurable if desired
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Ok(
            Texture {
                gl_ctx: gl_ctx,
                id: tex_id,
                tex_cfg: tex_cfg,
            }
        )
    }

    pub fn cubemap_from_files(gl_ctx: Rc<gl::Gl>, paths: CubemapPaths, tex_cfg: TexConfig) -> Result<Texture, TextureError> {
        tex_cfg.validate()?;
        let images = paths.cubemap_entries();
        let mut tex_id: u32 = 0;
        unsafe {
            gl_ctx.GenTextures(1, &mut tex_id);
            gl_ctx.BindTexture(tex_cfg.tex_type as u32, tex_id);
            
            // Load texture into memory.
            // TODO: not all images are RGB. Switch based on the image properties and error out if invalid pix format is used.
            // Right now, we don't intfer anything and have the user specify the data format. Maybe make this safer?
            for (img_path, cm_type) in images {
                let dyn_img = image::open(img_path)?;
                dyn_img.flipv();
                let rgb_image = dyn_img.to_rgb();

                let width = rgb_image.width() as i32;
                let height = rgb_image.height() as i32;
                let bytes = rgb_image.into_vec();
                // essentially for a cube-map we need to do this 6 times.
                gl_ctx.TexImage2D(cm_type as u32, 0, tex_cfg.internal_fmt as i32, width, height, 0, tex_cfg.pix_data_fmt as u32, tex_cfg.pix_type_fmt as u32, bytes.as_ptr() as *const _);
            }

            // Set texture wrap/filtering settings for _current_ texture
            // TODO: Make this configurable if desired
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_WRAP_R, gl::REPEAT as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        Ok(Texture {
            gl_ctx,
            id: tex_id,
            tex_cfg: tex_cfg,
        })
    }

    // todo: this probably need a configuration structure as framebuffers can be configured
    // in multiple buffer sizes and styles
    pub(crate) fn from_framebuffer(gl_ctx: Rc<gl::Gl>, width: i32, height: i32, tex_cfg: TexConfig, attachment: FrameBufferAttachment) -> Texture {
        let mut tex_id: u32 = 0;
        unsafe {
            gl_ctx.GenTextures(1, &mut tex_id);
            gl_ctx.BindTexture(tex_cfg.tex_type as u32, tex_id);

            // NULL here since we're binding to the current frame buffer.
            // make type configurable, since framebuffer types can be configurable
            gl_ctx.TexImage2D(tex_cfg.tex_type as u32, 0, tex_cfg.internal_fmt as i32, width, height, 0, tex_cfg.pix_data_fmt as u32, tex_cfg.pix_type_fmt as u32,  ptr::null());

            // Set texture filtering for _current_ texture
            // todo: make configurable, just like in from_file
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl_ctx.TexParameteri(tex_cfg.tex_type as u32, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // configure texture onto framebuffer.
            // make attachment parameter configurable
            gl_ctx.FramebufferTexture2D(gl::FRAMEBUFFER, attachment.into(), tex_cfg.tex_type as u32, tex_id, 0);
        }

        Texture {
            gl_ctx: gl_ctx,
            id: tex_id,
            tex_cfg: tex_cfg,
        }
    }

    pub fn bind(&self, tex_unit: TextureUnit) {
        unsafe {
            self.gl_ctx.ActiveTexture(tex_unit as u32);
            self.gl_ctx.BindTexture(self.tex_cfg.tex_type as u32, self.id);
        }
    }
}

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Loading a texture into OpenGL failed: {0}")]
    OpenGLError(OpenGLError),
    #[error("Loading the texture image failed: {0}")]
    ImageError(image::ImageError),
    #[error("Loading the texture file failed: {0}")]
    IOError(io::Error),
    #[error("Bad 2D texture configuration generated")]
    BadTextureConfig,
}

impl From<OpenGLError> for TextureError {
    fn from(err: OpenGLError) -> Self {
        TextureError::OpenGLError(err)
    }
}

impl From<image::ImageError> for TextureError {
    fn from(err: image::ImageError) -> Self {
        TextureError::ImageError(err)
    }
}

impl From<io::Error> for TextureError {
    fn from(err: io::Error) -> Self {
        TextureError::IOError(err)
    }
}