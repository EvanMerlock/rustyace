use crate::gl;
use std::collections::HashMap;
use std::rc::Rc;
use crate::types::*;

pub struct AttributeProperties {
    attrib_size: AttributeComponentSize,
    attrib_type: GLType,
    normalized: bool,
    stride: i32,
    offset: u32,
}

impl AttributeProperties {
    pub fn new(size: AttributeComponentSize, attrib_type: GLType, normalized: bool, stride: i32, offset: u32) -> AttributeProperties {

        let compute_size = attrib_type.sizeof();

        AttributeProperties {
            attrib_size: size,
            attrib_type: attrib_type,
            normalized: normalized,
            stride: stride * (compute_size as i32),
            offset: offset * (compute_size as u32),
        }
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