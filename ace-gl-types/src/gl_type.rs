use std::mem;
use crate::gl;

#[derive(Debug, Clone, Copy)]
pub enum GLType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed,
}

impl GLType {
    pub fn sizeof(&self) -> usize {
        match self {
            GLType::Byte                => mem::size_of::<i8>(),
            GLType::UnsignedByte        => mem::size_of::<u8>(),
            GLType::Short               => mem::size_of::<i16>(),
            GLType::UnsignedShort       => mem::size_of::<u16>(),
            GLType::Int                 => mem::size_of::<i32>(),
            GLType::UnsignedInt         => mem::size_of::<u32>(),
            GLType::HalfFloat           => mem::size_of::<i16>(),
            GLType::Float               => mem::size_of::<f32>(),
            GLType::Double              => mem::size_of::<f64>(),
            GLType::Fixed               => unimplemented!(),
        }
    }
}

impl Into<u32> for GLType {
    fn into(self) -> u32 {
        match self {
            GLType::Byte                => gl::BYTE,
            GLType::UnsignedByte        => gl::UNSIGNED_BYTE,
            GLType::Short               => gl::SHORT,
            GLType::UnsignedShort       => gl::UNSIGNED_SHORT,
            GLType::Int                 => gl::INT,
            GLType::UnsignedInt         => gl::UNSIGNED_INT,
            GLType::HalfFloat           => gl::HALF_FLOAT,
            GLType::Float               => gl::FLOAT,
            GLType::Double              => gl::DOUBLE,
            GLType::Fixed               => gl::FIXED,
        }
    }
}