use std::rc::Rc;
use crate::types::*;
use crate::gl;

pub struct RenderBuffer {
    gl_ctx: Rc<gl::Gl>, 
    id: u32,
    int_str: InternalStorage,
    attachment: FrameBufferAttachment,
}

impl RenderBuffer {
    fn bind(&self) {
        unsafe {
            self.gl_ctx.BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl_ctx.BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }

    pub fn from_framebuffer(gl_ctx: Rc<gl::Gl>, int_str: InternalStorage, width: i32, height: i32, attachment: FrameBufferAttachment) -> RenderBuffer {
        let mut rbo_id = 0;
        unsafe {
            gl_ctx.GenRenderbuffers(1, &mut rbo_id);
            gl_ctx.BindRenderbuffer(gl::RENDERBUFFER, rbo_id);
            gl_ctx.RenderbufferStorage(gl::RENDERBUFFER, int_str as u32, width, height);
            gl_ctx.FramebufferRenderbuffer(gl::FRAMEBUFFER, attachment.clone().into(), gl::RENDERBUFFER, rbo_id);
        }
        
        RenderBuffer {
            gl_ctx: gl_ctx,
            id: rbo_id,
            int_str: int_str,
            attachment: attachment
        }
    }
}