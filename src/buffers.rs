use crate::gl;
use crate::renderable::Model;
use std::collections::HashMap;
use std::rc::Rc;
use std::mem;
use crate::types::*;

// TODO:
// Buffer objects/shaders are GLOBAL STATE.
// We need a global lock for each type of buffer and shader programs, and then have buffers switch between a unbound and bound struct/state
// This allows us to prevent mistakes from binding buffers incorrectly (rebinding over a buffer before it's been dropped)


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

pub struct VertexArrayObj {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
    attributes: HashMap<u32, AttributeProperties>,
}

impl VertexArrayObj {
    pub fn new(gl_ctx: Rc<gl::Gl>) -> VertexArrayObj {
        let mut gl_id: u32 = 0;
        unsafe {
            gl_ctx.GenVertexArrays(1, &mut gl_id);
        }
        VertexArrayObj {
            gl_ctx: gl_ctx,
            id: gl_id,
            attributes: HashMap::new()
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl_ctx.BindVertexArray(self.id)
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl_ctx.BindVertexArray(0)
        }
    }

    pub fn configure_index(&mut self, index: u32, prop: AttributeProperties) {
        unsafe {
            if prop.normalized {
                self.gl_ctx.VertexAttribPointer(index, prop.attrib_size as i32, prop.attrib_type.into(), gl::TRUE, prop.stride, prop.offset as *const _);

            } else {
                self.gl_ctx.VertexAttribPointer(index, prop.attrib_size as i32, prop.attrib_type.into(), gl::FALSE, prop.stride, prop.offset as *const _);
            }
            self.gl_ctx.EnableVertexAttribArray(index);
        }
        self.attributes.insert(index, prop);
    }
}
pub struct AttributeProperties {
    attrib_size: AttributeComponentSize,
    attrib_type: GLType,
    normalized: bool,
    stride: i32,
    offset: u32,
}

impl AttributeProperties {
    pub fn new(size: AttributeComponentSize, attrib_type: GLType, normalized: bool, stride: i32, offset: u32) -> AttributeProperties {

        let compute_size = match attrib_type {
            GLType::Byte => mem::size_of::<i8>(),
            GLType::Double => mem::size_of::<f64>(),
            GLType::Fixed => mem::size_of::<i32>(),
            GLType::Float => mem::size_of::<f32>(),
            GLType::HalfFloat => mem::size_of::<u16>(),
            GLType::Int => mem::size_of::<i32>(),
            GLType::Short => mem::size_of::<i16>(),
            GLType::UnsignedByte => mem::size_of::<u8>(),
            GLType::UnsignedInt => mem::size_of::<u32>(),
            GLType::UnsignedShort => mem::size_of::<u16>(),
        } as u32;

        AttributeProperties {
            attrib_size: size,
            attrib_type: attrib_type,
            normalized: normalized,
            stride: stride * (compute_size as i32),
            offset: offset * compute_size,
        }
    }
}

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