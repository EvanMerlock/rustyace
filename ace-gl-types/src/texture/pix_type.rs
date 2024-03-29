use crate::gl;

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum PixelDataFormat {
    R            = gl::RED as isize,
    G            = gl::GREEN as isize,
    B            = gl::BLUE as isize,
    RG           = gl::RG as isize,
    RGB          = gl::RGB as isize,
    BGR          = gl::BGR as isize,
    RGBA         = gl::RGBA as isize,
    BGRA         = gl::BGRA as isize,
    Int_R        = gl::RED_INTEGER as isize,
    Int_G        = gl::GREEN_INTEGER as isize,
    Int_B        = gl::BLUE_INTEGER as isize,
    Int_RG       = gl::RG_INTEGER as isize,
    Int_RGB      = gl::RGB_INTEGER as isize,
    Int_BGR      = gl::BGR_INTEGER as isize,
    Int_RGBA     = gl::RGBA_INTEGER as isize,
    Int_BGRA     = gl::BGRA_INTEGER as isize,
    Stencil      = gl::STENCIL_INDEX as isize,
    Depth        = gl::DEPTH_COMPONENT as isize,
    DepthStencil = gl::DEPTH_STENCIL as isize,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum PixelDataType {
    UnsignedByte        = gl::UNSIGNED_BYTE as isize,
    Byte                = gl::BYTE as isize,
    UnsignedShort       = gl::UNSIGNED_SHORT as isize,
    Short               = gl::SHORT as isize,
    UnsignedInt         = gl::UNSIGNED_INT as isize,
    Int                 = gl::INT as isize,
    Float               = gl::FLOAT as isize,
    UnsignedByte332     = gl::UNSIGNED_BYTE_3_3_2 as isize,
    UnsignedByte233Rev  = gl::UNSIGNED_BYTE_2_3_3_REV as isize,
    UnsignedShort565    = gl::UNSIGNED_SHORT_5_6_5 as isize,
    UnsignedShort565Rev = gl::UNSIGNED_SHORT_5_6_5_REV as isize,
    UnsignedShort4444   = gl::UNSIGNED_SHORT_4_4_4_4 as isize,
    UnsignedShort4444Rev = gl::UNSIGNED_SHORT_4_4_4_4_REV as isize,
    UnsignedShort5551   = gl::UNSIGNED_SHORT_5_5_5_1 as isize,
    UnsignedShort1555Rev = gl::UNSIGNED_SHORT_1_5_5_5_REV as isize,
    UnsignedInt8888     = gl::UNSIGNED_INT_8_8_8_8 as isize,
    UnsignedInt8888Rev  = gl::UNSIGNED_INT_8_8_8_8_REV as isize,
    UnsignedInt1010102  = gl::UNSIGNED_INT_10_10_10_2 as isize,
    UnsignedInt2101010Rev = gl::UNSIGNED_INT_2_10_10_10_REV as isize,
}