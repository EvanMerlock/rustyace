use super::UniformType;
use super::gl;

mod signed {
    use super::*;
    impl UniformType for i32 {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform1i(loc, *self);
            }
        }
    }
    
    impl UniformType for nalgebra::Vector2<i32> {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform2i(loc, self[0], self[1]);
            }
        }
    }
    
    impl UniformType for nalgebra::Vector3<i32> {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform3i(loc, self[0], self[1], self[2]);
            }
        }
    }
    
    impl UniformType for nalgebra::Vector4<i32> {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform4i(loc, self[0], self[1], self[2], self[3]);
            }
        }
    }
    
    impl UniformType for &[i32] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform1iv(loc, self.len() as i32, self.as_ptr());
            }
        }
    }
    
    impl UniformType for &[nalgebra::Vector2<i32>] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform2iv(loc, self.len() as i32, self.as_ptr() as *const _);
            }
        }
    }
    
    impl UniformType for &[nalgebra::Vector3<i32>] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform3iv(loc, self.len() as i32, self.as_ptr() as *const _);
            }
        }
    }
    
    impl UniformType for &[nalgebra::Vector4<i32>] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform4iv(loc, self.len() as i32, self.as_ptr() as *const _);
            }
        }
    }
}

mod unsigned {
    use super::*;
    impl UniformType for u32 {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform1ui(loc, *self);
            }
        }
    }
    
    impl UniformType for nalgebra::Vector2<u32> {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform2ui(loc, self[0], self[1]);
            }
        }
    }
    
    impl UniformType for nalgebra::Vector3<u32> {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform3ui(loc, self[0], self[1], self[2]);
            }
        }
    }
    
    impl UniformType for nalgebra::Vector4<u32> {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform4ui(loc, self[0], self[1], self[2], self[3]);
            }
        }
    }
    
    impl UniformType for &[u32] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform1uiv(loc, self.len() as i32, self.as_ptr());
            }
        }
    }
    
    impl UniformType for &[nalgebra::Vector2<u32>] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform2uiv(loc, self.len() as i32, self.as_ptr() as *const _);
            }
        }
    }
    
    impl UniformType for &[nalgebra::Vector3<u32>] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform3uiv(loc, self.len() as i32, self.as_ptr() as *const _);
            }
        }
    }
    
    impl UniformType for &[nalgebra::Vector4<u32>] {
        fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
            unsafe {
                gl_ctx.Uniform4uiv(loc, self.len() as i32, self.as_ptr() as *const _);
            }
        }
    }
}

pub use self::signed::*;
pub use self::unsigned::*;