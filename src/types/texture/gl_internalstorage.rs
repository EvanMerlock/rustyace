use crate::gl;

#[allow(non_camel_case_types)]
pub enum InternalStorage {
    Depth                       = gl::DEPTH_COMPONENT as isize,
    DepthStencil                = gl::DEPTH_STENCIL as isize,
    Stencil                     = gl::STENCIL_INDEX as isize,

    Red                         = gl::RED as isize,
    Green                       = gl::GREEN as isize,
    Blue                        = gl::BLUE as isize,
    RG                          = gl::RG as isize,
    RGB                         = gl::RGB as isize,
    RGBA                        = gl::RGBA as isize,

    Red8                        = gl::R8 as isize,
    Norm_Red8                   = gl::R8_SNORM as isize,
    Red16                       = gl::R16 as isize,
    Norm_Red16                  = gl::R16_SNORM as isize,
    RedGreen8                   = gl::RG8 as isize,
    Norm_RedGreen8              = gl::RG8_SNORM as isize,

    RedGreen16                  = gl::RG16 as isize,
    Norm_RedGreen16             = gl::RG16_SNORM as isize,

    Red3Green3Blue2             = gl::R3_G3_B2 as isize,

    RedGreenBlue4               = gl::RGB4 as isize,
    RedGreenBlue5               = gl::RGB5 as isize,
    RedGreenBlue8               = gl::RGB8 as isize,
    Norm_RedGreenBlue8          = gl::RGB8_SNORM as isize,
    RedGreenBlue10              = gl::RGB10 as isize,
    RedGreenBlue12              = gl::RGB12 as isize,
    Norm_RedGreenBlue16         = gl::RGB16_SNORM as isize,

    RedGreenBlueAlpha2          = gl::RGBA2 as isize,
    RedGreenBlueAlpha4          = gl::RGBA4 as isize,
    RedGreenBlue5Alpha1         = gl::RGB5_A1 as isize,
    RedGreenBlueAlpha8          = gl::RGBA8 as isize,
    Norm_RedGreenBlueAlpha8     = gl::RGBA8_SNORM as isize,
    RedGreenBlue10Alpha2        = gl::RGB10_A2 as isize,
    UInt_RedGreenBlue10Alpha2   = gl::RGB10_A2UI as isize,
    RedGreenBlueAlpha12         = gl::RGBA12 as isize,
    RedGreenBlueAlpha16         = gl::RGBA16 as isize,

    Std_RedGreenBlue8           = gl::SRGB8 as isize,
    Std_RedGreenBlueAlpha8      = gl::SRGB8_ALPHA8 as isize,

    Float_Red16                 = gl::R16F as isize,
    Float_RedGreen16            = gl::RG16F as isize,
    Float_RedGreenBlue16        = gl::RGB16F as isize,
    Float_RedGreenBlueAlpha16   = gl:: RGBA16F as isize,

    Float_Red32                 = gl::R32F as isize,
    Float_RedGreen32            = gl::RG32F as isize,
    Float_RedGreenBlue32        = gl::RGB32F as isize,
    Float_RedGreenBlueAlpha32   = gl::RGBA32F as isize,

    Float_Red11Green11Blue10    = gl::R11F_G11F_B10F as isize,
    
    RedGreenBlue9Shared5        = gl::RGB9_E5 as isize,

    Int_Red8                    = gl::R8I as isize,
    UInt_Red8                   = gl::R8UI as isize,
    Int_Red16                   = gl::R16I as isize,
    UInt_Red16                  = gl::R16UI as isize,
    Int_Red32                   = gl::R32I as isize,
    UInt_Red32                  = gl::R32UI as isize,

    Int_RedGreen8               = gl::RG8I as isize,
    UInt_RedGreen8              = gl::RG8UI as isize,
    Int_RedGreen16              = gl::RG16I as isize,
    UInt_RedGreen16             = gl::RG16UI as isize,
    Int_RedGreen32              = gl::RG32I as isize,
    UInt_RedGreen32             = gl::RG32UI as isize,

    Int_RedGreenBlue8           = gl::RGB8I as isize,
    UInt_RedGreenBlue8          = gl::RGB8UI as isize,
    Int_RedGreenBlue16          = gl::RGB16I as isize,
    UInt_RedGreenBlue16         = gl::RGB16UI as isize,
    Int_RedGreenBlue32          = gl::RGB32I as isize,
    UInt_RedGreenBlue32         = gl::RGB32UI as isize,

    Int_RedGreenBlueAlpha8      = gl::RGBA8I as isize,
    UInt_RedGreenBlueAlpha8     = gl::RGBA8UI as isize,
    Int_RedGreenBlueAlpha16     = gl::RGBA16I as isize,
    UInt_RedGreenBlueAlpha16    = gl::RGBA16UI as isize,
    Int_RedGreenBlueAlpha32     = gl::RGBA32I as isize,
    UInt_RedGreenBlueAlpha32    = gl::RGBA32UI as isize,

    Depth16                     = gl::DEPTH_COMPONENT16 as isize,
    Depth24                     = gl::DEPTH_COMPONENT24 as isize,
    Depth32                     = gl::DEPTH_COMPONENT32 as isize,
    Float_Depth32               = gl::DEPTH_COMPONENT32F as isize,
    Depth24Stencil8             = gl::DEPTH24_STENCIL8 as isize,
    Float_Depth32Stencil8       = gl::DEPTH32F_STENCIL8 as isize,
    Stencil8                    = gl::STENCIL_INDEX8 as isize,

}