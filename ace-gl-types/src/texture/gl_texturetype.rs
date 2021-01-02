use crate::gl;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TextureType {
    Texture2D               = gl::TEXTURE_2D as isize,
    ProxyTexture2D          = gl::PROXY_TEXTURE_2D as isize,

    Texture1DArray          = gl::TEXTURE_1D_ARRAY as isize,
    ProxyTexture1DArray     = gl::PROXY_TEXTURE_1D_ARRAY as isize,

    TextureRectangle        = gl::TEXTURE_RECTANGLE as isize,
    ProxyTextureRectangle   = gl::PROXY_TEXTURE_RECTANGLE as isize,

    TextureCubeMap          = gl::TEXTURE_CUBE_MAP as isize,
    ProxyTextureCubeMap     = gl::PROXY_TEXTURE_CUBE_MAP as isize,
}