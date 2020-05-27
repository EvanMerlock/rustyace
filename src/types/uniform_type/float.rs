use super::UniformType;
use super::gl;

impl UniformType for f32 {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1f(loc, *self);
        }
    }
}

impl UniformType for nalgebra::Vector2<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2f(loc, self[0], self[1]);
        }
    }
}

impl UniformType for nalgebra::Vector3<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3f(loc, self[0], self[1], self[2]);
        }
    }
}

impl UniformType for nalgebra::Vector4<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4f(loc, self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformType for &[f32] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1fv(loc, self.len() as i32, self.as_ptr());
        }
    }
}

impl UniformType for &[nalgebra::Vector2<f32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2fv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector3<f32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3fv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector4<f32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4fv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}