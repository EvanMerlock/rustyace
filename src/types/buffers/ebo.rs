use crate::gl;
use std::rc::Rc;
use crate::types::*;

pub struct ElementArrayObj {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
}

impl ElementArrayObj {
    pub fn new(gl_ctx: Rc<gl::Gl>) -> ElementArrayObj {
        let mut gl_id: u32 = 0;
        unsafe {
            gl_ctx.GenBuffers(1, &mut gl_id);
        }
        ElementArrayObj {
            gl_ctx: gl_ctx,
            id: gl_id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl_ctx.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl_ctx.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn copy_to_buffer<T: TypedBuffer>(&self, indicies: T, draw_mode: DrawMode) {
        self.bind();
        unsafe {
            self.gl_ctx.BufferData(gl::ELEMENT_ARRAY_BUFFER, indicies.size() as isize, indicies.ref_ptr(), draw_mode as u32);
        }
    }
}