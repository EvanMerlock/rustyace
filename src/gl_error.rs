use thiserror::Error;
use super::shaders;

#[derive(Error, Debug)]
pub enum OpenGLError {
    #[error("Shader failed to compile: {0}")]
    CompileError(String),
    #[error("Shader program failed to link: {1}")]
    LinkerError(shaders::ShaderProgram, String),
    #[error("The shader program already contained a shader of the same type: {0}")]
    ProgramAlreadyContainedShader(shaders::ShaderType),
}