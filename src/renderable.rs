use super::gl_error;
use super::gl;
use crate::types::*;
use std::rc::Rc;
use std::mem;
use std::ptr;
use super::shaders::CompiledShaderProgram;

pub const TRI_VERTICES: [f32; 32] = [
        // positions      // colors        // texture coords
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
       -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
       -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left 
];

pub const TRI_INDICES: [u32; 6] = [
    0, 1, 3,   // first triangle
    1, 2, 3    // second triangle
];

pub const CUBE_VERTICES: [f32; 192] = [
    // positions       // colors       // tex coords
    -1.0, -1.0,  1.0,  1.0, 0.0, 0.0,  0.0, 0.0, // front
     1.0, -1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
     1.0,  1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
    -1.0,  1.0,  1.0,  1.0, 0.0, 0.0,  0.0, 1.0,

    -1.0,  1.0,  1.0,  1.0, 0.0, 0.0,  0.0, 0.0, // top
     1.0,  1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
     1.0,  1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
    -1.0,  1.0, -1.0,  1.0, 0.0, 0.0,  0.0, 1.0,

     1.0, -1.0, -1.0,  1.0, 0.0, 0.0,  0.0, 0.0, // back
    -1.0, -1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
    -1.0,  1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
     1.0,  1.0, -1.0,  1.0, 0.0, 0.0,  0.0, 1.0,

    -1.0, -1.0, -1.0,  1.0, 0.0, 0.0,  0.0, 0.0, //bottom
     1.0, -1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
     1.0, -1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
    -1.0, -1.0,  1.0,  1.0, 0.0, 0.0,  0.0, 1.0,

    -1.0, -1.0, -1.0,  1.0, 0.0, 0.0,  0.0, 0.0, // left
    -1.0, -1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
    -1.0,  1.0,  1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
    -1.0,  1.0, -1.0,  1.0, 0.0, 0.0,  0.0, 1.0,

     1.0, -1.0,  1.0,  1.0, 0.0, 0.0,  0.0, 0.0, // right
     1.0, -1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 0.0,
     1.0,  1.0, -1.0,  1.0, 0.0, 0.0,  1.0, 1.0,
     1.0,  1.0,  1.0,  1.0, 0.0, 0.0,  0.0, 1.0,
];

pub const CUBE_INDICES: [u32; 36] = [
    // front
    0,  1,  2,
    2,  3,  0,
    // top
    4,  5,  6,
    6,  7,  4,
    // back
    8,  9, 10,
    10, 11,  8,
    // bottom
    12, 13, 14,
    14, 15, 12,
    // left
    16, 17, 18,
    18, 19, 16,
    // right
    20, 21, 22,
    22, 23, 20,
];

// TODO: Figure out if we need to split models into meshes (we probably do)
// And the best way to communicate data to the GPU.
// With nalgebra, we might be able to augment matricies by row (since matricies are column-major) in order to add more information
// So then we could have separate color/lighting/texture matricies
pub trait Model {
    fn get_vertices(&self)  -> &Vec<f32>;
    fn vertices_len(&self)  -> i32;
    fn vertices_size(&self) -> isize;
    fn get_indices(&self)   -> &Vec<u32>;
    fn indices_len(&self)   -> i32;
    fn get_shader(&self)    -> &Rc<CompiledShaderProgram>;
}

pub struct ResidentModel {
    vertices: Vec<f32>,
    vert_len: i32,
    indices: Vec<u32>,
    index_len: i32,
    shader: Rc<CompiledShaderProgram>,
}

impl ResidentModel {
    pub fn new(vert: &[f32], indices: &[u32], shdr_prog: Rc<CompiledShaderProgram>) -> ResidentModel {
        let vertices = vert.to_vec();
        let indexes = indices.to_vec();
        let vert_len = vertices.len() as i32;
        let index_len = indexes.len() as i32;
        ResidentModel {
            vertices: vertices,
            vert_len: vert_len,
            indices: indexes,
            index_len: index_len,
            shader: shdr_prog,
        }
    }
}

impl Model for ResidentModel {
    fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    fn vertices_len(&self) -> i32 {
        self.vert_len
    }

    fn vertices_size(&self) -> isize {
        (self.vertices_len() * mem::size_of::<f32>() as i32) as isize
    }

    fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }

    fn indices_len(&self) -> i32 {
        self.index_len
    }

    fn get_shader(&self) -> &Rc<CompiledShaderProgram> {
        &self.shader
    }
}

pub struct Renderable {
    gl_ctx: Rc<gl::Gl>,
    model: Rc<dyn Model>,
    vao: VertexArrayObj,
    vbo: VertexBufferObj,
    ebo: ElementArrayObj,
}

impl Renderable {

    pub fn new(gl_ctx: Rc<gl::Gl>, model: Rc<dyn Model>, attrib_spec: impl Fn(&mut VertexArrayObj) -> ()) -> Result<Renderable, gl_error::OpenGLError> {
        let mut vertex_array = VertexArrayObj::new(gl_ctx.clone());
        let vertex_buffer = VertexBufferObj::new(gl_ctx.clone());
        let element_buffer = ElementArrayObj::new(gl_ctx.clone());

        vertex_array.bind();
        vertex_buffer.bind();
        element_buffer.bind();
        vertex_buffer.copy_to_buffer(&model, DrawMode::StaticDraw);
        element_buffer.copy_to_buffer(&model, DrawMode::StaticDraw);
        attrib_spec(&mut vertex_array);
        Ok(Renderable {
            gl_ctx: gl_ctx,
            model: model,
            vao: vertex_array,
            vbo: vertex_buffer,
            ebo: element_buffer,
        })    
    }

    pub fn render(&self, array_dmode: GLMode, uniform_set: impl Fn(&CompiledShaderProgram) -> ()) -> Result<(), gl_error::OpenGLError> {
        let shader = self.model.get_shader();
        shader.use_program();

        // Sets per-frame uniforms
        // For example, MVP matricies (specifically view and projection, since model should be passed into the program through the model data)
        uniform_set(shader.as_ref());

        self.vao.bind();
        unsafe {
            self.gl_ctx.DrawElements(array_dmode as u32, self.model.indices_len(), GLType::UnsignedInt.into(), ptr::null());
        }
        Ok(())
    }
}