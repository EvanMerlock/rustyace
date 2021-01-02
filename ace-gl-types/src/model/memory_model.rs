use crate::types::*;
use crate::gl;
use std::rc::Rc;

pub struct ResidentModel {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    shader: Rc<CompiledShaderProgram>,
    vao: VertexArrayObj,
    vbo: VertexBufferObj,
    eao: ElementArrayObj,
}

impl ResidentModel {
    pub fn new(gl_ctx: Rc<gl::Gl>, vert: &[f32], indices: &[u32], shdr_prog: Rc<CompiledShaderProgram>, attrib_spec: impl Fn(&mut VertexArrayObj) -> ()) -> ResidentModel {
        let vertices = vert.to_vec();
        let indexes = indices.to_vec();

        let mut vertex_array = VertexArrayObj::new(gl_ctx.clone());
        let vertex_buffer = VertexBufferObj::new(gl_ctx.clone());
        let element_buffer = ElementArrayObj::new(gl_ctx.clone());

        vertex_array.bind();
        vertex_buffer.bind();
        element_buffer.bind();
        vertex_buffer.copy_to_buffer(vert, DrawMode::StaticDraw);
        element_buffer.copy_to_buffer(indices, DrawMode::StaticDraw);
        attrib_spec(&mut vertex_array);

        ResidentModel {
            vertices: vertices,
            indices: indexes,
            shader: shdr_prog,
            vao: vertex_array,
            vbo: vertex_buffer,
            eao: element_buffer,
        }
    }
}

impl Model for ResidentModel {
    fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }

    fn get_shader(&self) -> &Rc<CompiledShaderProgram> {
        &self.shader
    }

    fn get_vert_array_obj(&self) -> &VertexArrayObj {
        &self.vao
    }

    fn get_vert_buffer_obj(&self) -> &VertexBufferObj {
        &self.vbo
    }

    fn get_elem_array_obj(&self) -> &ElementArrayObj {
        &self.eao
    }
}