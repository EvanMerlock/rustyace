use image;
use std::rc::Rc;
use crate::gl;
use std::path::Path;
use thiserror::Error;
use std::ptr;
use crate::types::*;

mod gl_internalstorage;
mod pix_type;
pub use self::gl_internalstorage::*;
pub use self::pix_type::*;

pub struct Texture {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
}

impl Texture {
    pub fn from_file<P: AsRef<Path>>(gl_ctx: Rc<gl::Gl>, path: P, internal_buffer: InternalStorage) -> Result<Texture, TextureError> {
        let mut dyn_img = image::open(path)?;
        dyn_img = dyn_img.flipv();
        let rgb_image = dyn_img.to_rgb();
        let mut tex_id: u32 = 0;
        unsafe {
            gl_ctx.GenTextures(1, &mut tex_id);
            // TODO: We should probably allow TEXTURE_2D to be specified by the caller.
            gl_ctx.BindTexture(gl::TEXTURE_2D, tex_id);

            // Set texture wrap/filtering settings for _current_ texture
            // TODO: Make this configurable if desired
            gl_ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl_ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl_ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl_ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            // Load texture into memory.
            // TODO: not all images are RGB. Switch based on the image properties and error out if invalid pix format is used.
            // Right now, we just turn the image directly into RGB. Look into matching on DynamicImage.
            let width = rgb_image.width() as i32;
            let height = rgb_image.height() as i32;
            let bytes = rgb_image.into_vec();
            gl_ctx.TexImage2D(gl::TEXTURE_2D, 0, internal_buffer as i32, width, height, 0, gl::RGB, GLType::UnsignedByte.into(), bytes.as_ptr() as *const _);
            gl_ctx.GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(
            Texture {
                gl_ctx: gl_ctx,
                id: tex_id
            }
        )
    }

    // todo: this probably need a configuration structure as framebuffers can be configured
    // in multiple buffer sizes and styles
    pub(crate) fn from_framebuffer(gl_ctx: Rc<gl::Gl>, width: i32, height: i32, internal_buffer: InternalStorage) -> Texture {
        let mut tex_id: u32 = 0;
        unsafe {
            gl_ctx.GenTextures(1, &mut tex_id);
            gl_ctx.BindTexture(gl::TEXTURE_2D, tex_id);

            // NULL here since we're binding to the current frame buffer.
            // make type configurable, since framebuffer types can be configurable
            gl_ctx.TexImage2D(gl::TEXTURE_2D, 0, internal_buffer as i32, width, height, 0, gl::RGB, GLType::UnsignedByte.into(),  ptr::null());

            // Set texture filtering for _current_ texture
            // todo: make configurable, just like in from_file
            gl_ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl_ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // configure texture onto framebuffer.
            // make attachment parameter configurable
            gl_ctx.FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, tex_id, 0);
        }

        Texture {
            gl_ctx: gl_ctx,
            id: tex_id,
        }
    }

    pub fn bind(&self, tex_unit: TextureUnit) {
        unsafe {
            self.gl_ctx.ActiveTexture(tex_unit as u32);
            self.gl_ctx.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Loading a texture into OpenGL failed: {0}")]
    OpenGLError(OpenGLError),
    #[error("Loading the texture image failed: {0}")]
    ImageError(image::ImageError),
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