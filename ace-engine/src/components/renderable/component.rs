use crate::{gl};
use crate::types::*;
use std::rc::Rc;
use std::ptr;

pub struct Renderable {
    gl_ctx: Rc<gl::Gl>,
    model: Rc<dyn Model>,
}

impl Renderable {

    pub fn new(gl_ctx: Rc<gl::Gl>, model: Rc<dyn Model>) -> Result<Renderable, OpenGLError> {
        Ok(Renderable {
            gl_ctx: gl_ctx,
            model: model,
        })    
    }

    pub fn render(&self, array_dmode: GLMode, uniform_set: impl Fn(&CompiledShaderProgram) -> ()) -> Result<(), OpenGLError> {
        let shader = self.model.get_shader();
        shader.use_program();

        // Sets per-frame uniforms
        // For example, MVP matricies (specifically view and projection, since model should be passed into the program through the model data)
        uniform_set(shader.as_ref());

        self.model.get_vert_array_obj().bind();
        unsafe {
            self.gl_ctx.DrawElements(array_dmode as u32, self.model.get_indices().len() as i32, GLType::UnsignedInt.into(), ptr::null());
        }
        Ok(())
    }
}