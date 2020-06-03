use crate::gl;
use std::rc::Rc;
use crate::types::*;

pub struct VertexBufferObj {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
}

impl VertexBufferObj {
    pub fn new(gl_ctx: Rc<gl::Gl>) -> VertexBufferObj {
        let mut gl_id: u32 = 0;
        unsafe {
            gl_ctx.GenBuffers(1, &mut gl_id);
        }        
        VertexBufferObj {
            gl_ctx: gl_ctx,
            id: gl_id
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl_ctx.BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl_ctx.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn copy_to_buffer<T: TypedBuffer>(&self, verts: T, draw_mode: DrawMode) {
        self.bind();
        unsafe {
            self.gl_ctx.BufferData(gl::ARRAY_BUFFER, verts.size() as isize, verts.ref_ptr(), draw_mode as u32);
        }
    }
}