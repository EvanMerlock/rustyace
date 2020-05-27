use super::UniformType;
use super::gl;

impl UniformType for nalgebra::Matrix2<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix2fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix3<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix3fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix4<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix4fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix2x3<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix2x3fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix2x4<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix2x4fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix4x2<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix4x2fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix3x4<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix3x4fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

impl UniformType for nalgebra::Matrix4x3<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.UniformMatrix4x3fv(loc, 1, gl::FALSE, self.as_ptr());
        }
    }
}

// TODO: Necessary? Depends on if we compute view/model in-engine or in-shader
// Since cameras shouldn't switch on us that much, I think it's safe to do in-engine.
impl UniformType for nalgebra::Isometry2<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        let mat: nalgebra::Matrix3<f32> = self.to_homogeneous();
        mat.assign_to_current_program(gl_ctx, loc);
    }
}

impl UniformType for nalgebra::Isometry3<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        let mat: nalgebra::Matrix4<f32> = self.to_homogeneous();
        mat.assign_to_current_program(gl_ctx, loc);
    }
}