use std::rc::Rc;
use crate::types::*;
use crate::gl;

pub struct FrameBuffer {
    gl_ctx: Rc<gl::Gl>, 
    id: u32,
    textures: Vec<Texture>,
    render_buffers: Vec<RenderBuffer>,

}

impl FrameBuffer {
    pub fn new(gl_ctx: Rc<gl::Gl>) -> FrameBuffer {
        let mut fbo_id = 0;
        unsafe {
            gl_ctx.GenFramebuffers(1, &mut fbo_id);
        }
        FrameBuffer {
            gl_ctx: gl_ctx,
            id: fbo_id,
            textures: Vec::new(),
            render_buffers: Vec::new(),
        }
    }

    pub fn bind(&self, behavior: FrameBufferRDBehavior) {
        unsafe {
            self.gl_ctx.BindFramebuffer(behavior as u32, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl_ctx.BindFramebuffer(FrameBufferRDBehavior::RD as u32, 0);
        }
    }

    pub fn is_complete(&self) -> bool {
        unsafe {
            self.gl_ctx.CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
        }
    }

    pub fn attach_texture(&mut self, width: i32, height: i32, tex_cfg: TexConfig, attachment: FrameBufferAttachment) {
        let tex = Texture::from_framebuffer(self.gl_ctx.clone(), width, height, tex_cfg, attachment);
        self.textures.push(tex);
    }

    pub fn attach_renderbuffer(&mut self, width: i32, height: i32, internal_storage: InternalStorage, attachment: FrameBufferAttachment) {
        let rbo = RenderBuffer::from_framebuffer(self.gl_ctx.clone(), internal_storage, width, height, attachment);
        self.render_buffers.push(rbo);
    }

    pub fn get_texture(&self, idx: usize) -> &Texture {
        &self.textures[idx]
    }
}

pub enum FrameBufferRDBehavior {
    ReadOnly    = gl::READ_FRAMEBUFFER as isize,
    DrawOnly    = gl::DRAW_FRAMEBUFFER as isize,
    RD          = gl::FRAMEBUFFER as isize,
}