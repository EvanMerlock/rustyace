use std::rc::Rc;
use crate::types::*;
use crate::gl;

pub struct RenderBuffer {
    gl_ctx: Rc<gl::Gl>, 
    id: u32,
}

impl RenderBuffer {
    fn new(gl_ctx: Rc<gl::Gl>) -> RenderBuffer {
        unimplemented!()
    }
}