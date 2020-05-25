use super::gl_error;
use super::gl;
use super::buffers;
use std::rc::Rc;
use std::mem;
use std::ptr;
use super::shaders::CompiledShaderProgram;

pub const TRI_VERTICES: [f32; 12] = [
    0.5,  0.5, 0.0,  // top right
    0.5, -0.5, 0.0,  // bottom right
   -0.5, -0.5, 0.0,  // bottom left
   -0.5,  0.5, 0.0   // top left 
];

pub const TRI_INDICES: [u32; 6] = [
    0, 1, 3,   // first triangle
    1, 2, 3    // second triangle
];

pub enum GLMode {
    Points                  = gl::POINTS as isize,
    LineStrip               = gl::LINE_STRIP as isize,
    LineLoop                = gl::LINE_LOOP as isize,
    Lines                   = gl::LINES as isize,
    LineStripAdjacency      = gl::LINE_STRIP_ADJACENCY as isize,
    LinesAdjacency          = gl::LINES_ADJACENCY as isize,
    TriangleStrip           = gl::TRIANGLE_STRIP as isize,
    TriangleFan             = gl::TRIANGLE_FAN as isize,
    Triangles               = gl::TRIANGLES as isize,
    TriangleStripAdjacency  = gl::TRIANGLE_STRIP_ADJACENCY as isize,
    TrianglesAdjacency      = gl::TRIANGLES_ADJACENCY as isize,
    Patches                 = gl::PATCHES as isize,
}

// TODO: Figure out if we need to split models into meshes (we probably do)
// And the best way to communicate data to the GPU.
// With nalgebra, we might be able to augment matricies by column in order to add more information
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
    vao: buffers::VertexArrayObj,
    vbo: buffers::VertexBufferObj,
    ebo: buffers::ElementArrayObj,
}

impl Renderable {

    pub fn new(gl_ctx: Rc<gl::Gl>, model: Rc<dyn Model>, attrib_spec: impl Fn(&mut buffers::VertexArrayObj) -> ()) -> Result<Renderable, gl_error::OpenGLError> {
        let mut vertex_array = buffers::VertexArrayObj::new(gl_ctx.clone());
        let vertex_buffer = buffers::VertexBufferObj::new(gl_ctx.clone());
        let element_buffer = buffers::ElementArrayObj::new(gl_ctx.clone());

        vertex_array.bind();
        vertex_buffer.bind();
        element_buffer.bind();
        vertex_buffer.copy_to_buffer(&model, buffers::DrawMode::StaticDraw);
        element_buffer.copy_to_buffer(&model, buffers::DrawMode::StaticDraw);
        attrib_spec(&mut vertex_array);
        Ok(Renderable {
            gl_ctx: gl_ctx,
            model: model,
            vao: vertex_array,
            vbo: vertex_buffer,
            ebo: element_buffer,
        })    
    }

    pub fn render(&self, array_dmode: GLMode) -> Result<(), gl_error::OpenGLError> {
        self.model.get_shader().use_program();
        self.vao.bind();
        unsafe {
            self.gl_ctx.DrawElements(array_dmode as u32, self.model.indices_len(), buffers::GLType::UnsignedInt.into(), ptr::null());
        }
        Ok(())
    }
}