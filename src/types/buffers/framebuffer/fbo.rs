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
    fn new(gl_ctx: Rc<gl::Gl>) -> FrameBuffer {
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

    fn bind(&self, behavior: FrameBufferRDBehavior) {
        unsafe {
            self.gl_ctx.BindFramebuffer(behavior as u32, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl_ctx.BindFramebuffer(FrameBufferRDBehavior::RD as u32, 0);
        }
    }

    fn is_complete(&self) -> bool {
        unsafe {
            self.gl_ctx.CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE
        }
    }

    fn attach_texture(&mut self, width: i32, height: i32, int_format: InternalStorage) {
        let tex = Texture::from_framebuffer(self.gl_ctx.clone(), width, height, int_format);
        self.textures.push(tex);
    }
}

enum FrameBufferRDBehavior {
    ReadOnly    = gl::READ_FRAMEBUFFER as isize,
    DrawOnly    = gl::DRAW_FRAMEBUFFER as isize,
    RD          = gl::FRAMEBUFFER as isize,
}