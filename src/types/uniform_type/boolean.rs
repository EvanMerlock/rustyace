use super::UniformType;
use super::gl;

impl UniformType for bool {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1i(loc, (*self) as i32);
        }
    }
}

impl UniformType for nalgebra::Vector2<bool> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2i(loc, self[0] as i32, self[1] as i32);
        }
    }
}

impl UniformType for nalgebra::Vector3<bool> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3i(loc, self[0] as i32, self[1] as i32, self[2] as i32);
        }
    }
}

impl UniformType for nalgebra::Vector4<bool> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4i(loc, self[0] as i32, self[1] as i32, self[2] as i32, self[3] as i32);
        }
    }
}