use super::gl_error;
use super::gl;
use super::buffers;
use std::rc::Rc;
use std::mem;
use super::shaders::CompiledShaderProgram;

pub const TRI_VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0,  0.5, 0.0
];

pub const TRI_INDICES: [i32; 3] = [
    0, 1, 2
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

pub trait Model {
    fn get_vertices(&self)  -> &Vec<f32>;
    fn vertices_len(&self)  -> i32;
    fn vertices_size(&self) -> isize;
    fn get_indices(&self)   -> &Vec<i32>;
    fn indices_len(&self)   -> i32;
    fn get_shader(&self)    -> &Rc<CompiledShaderProgram>;
}

pub struct ResidentModel {
    vertices: Vec<f32>,
    vert_len: i32,
    indices: Vec<i32>,
    index_len: i32,
    shader: Rc<CompiledShaderProgram>,
}

impl ResidentModel {
    pub fn new(vert: &[f32], indices: &[i32], shdr_prog: Rc<CompiledShaderProgram>) -> ResidentModel {
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

    fn get_indices(&self) -> &Vec<i32> {
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
    model: Rc<dyn Model>,
    vao: buffers::VertexArrayObj,
    vbo: buffers::VertexBufferObj,
}

impl Renderable {

    pub fn new(gl_ctx: &gl::Gl, model: Rc<dyn Model>, attrib_spec: impl Fn(&gl::Gl, &mut buffers::VertexArrayObj) -> ()) -> Result<Renderable, gl_error::OpenGLError> {
        let mut vertex_array = buffers::VertexArrayObj::new(gl_ctx);
        let vertex_buffer = buffers::VertexBufferObj::new(gl_ctx);

        vertex_array.bind(gl_ctx);
        vertex_buffer.bind(gl_ctx);
        vertex_buffer.copy_to_buffer(gl_ctx, &model, buffers::DrawMode::StaticDraw);
        attrib_spec(gl_ctx, &mut vertex_array);
        Ok(Renderable {
            model: model,
            vao: vertex_array,
            vbo: vertex_buffer,
        })    
    }

    pub fn render(&self, gl_ctx: &gl::Gl, array_dmode: GLMode) -> Result<(), gl_error::OpenGLError> {
        self.model.get_shader().use_program(gl_ctx);
        self.vao.bind(gl_ctx);
        unsafe {
            gl_ctx.DrawArrays(array_dmode as u32, 0, self.model.vertices_len());
        }
        Ok(())
    }
}