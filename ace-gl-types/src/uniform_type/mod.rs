use crate::gl;

pub trait UniformType {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32);
}

mod float;
mod int;
mod boolean;
mod matrix;

pub use self::float::*;
pub use self::int::*;
pub use self::boolean::*;
pub use self::matrix::*;