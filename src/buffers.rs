use crate::gl;
use crate::renderable::Model;
use std::collections::HashMap;
use std::rc::Rc;

pub enum DrawMode {
    StreamDraw = gl::STREAM_DRAW as isize,
    StreamRead = gl::STREAM_READ as isize,
    StreamCopy = gl::STREAM_COPY as isize,
    StaticDraw = gl::STATIC_DRAW as isize,
    StaticRead = gl::STATIC_READ as isize,
    StaticCopy = gl::STATIC_COPY as isize,
    DynamicDraw = gl::DYNAMIC_DRAW as isize,
    DynamicRead = gl::DYNAMIC_READ as isize,
    DynamicCopy = gl::DYNAMIC_COPY as isize,
}



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
            self._copy_to_buffer(vertices.get_vertices(), (vertices.vertices_len() as isize * vertices.vertices_size()), draw_mode);
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

#[derive(Debug, Clone, Copy)]
pub enum AttributeComponentSize {
    One   = 1,
    Two   = 2,
    Three = 3,
    Four  = 4
}

#[derive(Debug, Clone, Copy)]
pub enum GLType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed,
}

impl Into<u32> for GLType {
    fn into(self) -> u32 {
        match self {
            GLType::Byte                => gl::BYTE,
            GLType::UnsignedByte        => gl::UNSIGNED_BYTE,
            GLType::Short               => gl::SHORT,
            GLType::UnsignedShort       => gl::UNSIGNED_SHORT,
            GLType::Int                 => gl::INT,
            GLType::UnsignedInt         => gl::UNSIGNED_INT,
            GLType::HalfFloat           => gl::HALF_FLOAT,
            GLType::Float               => gl::FLOAT,
            GLType::Double              => gl::DOUBLE,
            GLType::Fixed               => gl::FIXED,

        }
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
        AttributeProperties {
            attrib_size: size,
            attrib_type: attrib_type,
            normalized: normalized,
            stride: stride,
            offset: offset,
        }
    }
}