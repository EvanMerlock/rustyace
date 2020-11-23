use std::ffi::c_void;
use crate::types::*;
pub trait TypedBuffer {
    fn get_gl_type(&self) -> GLType;
    fn length(&self) -> usize;
    fn size(&self) -> usize;
    fn ref_ptr(&self) -> *const c_void;
}

impl TypedBuffer for &[i32] {
    fn get_gl_type(&self) -> GLType {
        GLType::Int
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn size(&self) -> usize {
        self.get_gl_type().sizeof() * self.length()
    }

    fn ref_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }
}

impl TypedBuffer for &[u32] {
    fn get_gl_type(&self) -> GLType {
        GLType::UnsignedInt
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn size(&self) -> usize {
        self.get_gl_type().sizeof() * self.length()
    }

    fn ref_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }
}


impl TypedBuffer for &[f32] {
    fn get_gl_type(&self) -> GLType {
        GLType::Float
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn size(&self) -> usize {
        self.get_gl_type().sizeof() * self.length()
    }

    fn ref_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }
}


impl TypedBuffer for &[f64] {
    fn get_gl_type(&self) -> GLType {
        GLType::Double
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn size(&self) -> usize {
        self.get_gl_type().sizeof() * self.length()
    }

    fn ref_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }
}