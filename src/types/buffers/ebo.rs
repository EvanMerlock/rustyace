use crate::gl;
use crate::renderable::Model;
use std::collections::HashMap;
use std::rc::Rc;
use std::mem;
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

    pub fn copy_to_buffer(&self, vertices: &Rc<dyn Model>, draw_mode: DrawMode) {
        self.bind();
        unsafe {
            self._copy_to_buffer(vertices.get_indices(), vertices.indices_len() as isize * mem::size_of::<u32>() as isize, draw_mode);
        }
    }

    unsafe fn _copy_to_buffer(&self, vertices: &[u32], size: isize, draw_mode: DrawMode) {
        self.gl_ctx.BufferData(gl::ELEMENT_ARRAY_BUFFER, size, vertices.as_ptr() as *const _, draw_mode as u32);
    }
}