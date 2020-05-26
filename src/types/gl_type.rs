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