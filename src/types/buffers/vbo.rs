use crate::gl;
use crate::renderable::Model;
use std::collections::HashMap;
use std::rc::Rc;
use std::mem;
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

    pub fn copy_to_buffer(&self, vertices: &Rc<dyn Model>, draw_mode: DrawMode) {
        self.bind();
        unsafe {
            self._copy_to_buffer(vertices.get_vertices(), vertices.vertices_size(), draw_mode);
        }
    }

    unsafe fn _copy_to_buffer(&self, vertices: &[f32], size: isize, draw_mode: DrawMode) {
        self.gl_ctx.BufferData(gl::ARRAY_BUFFER, size, vertices.as_ptr() as *const _, draw_mode as u32);
    }
}