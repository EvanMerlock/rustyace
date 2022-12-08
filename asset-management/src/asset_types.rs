use ace_gl_types::ShaderType;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum AssetType {
    Model,
    Material,
    Texture,
    Cubemap,
    Audio,
    Shader(ShaderType),
}