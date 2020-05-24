use super::gl;
use super::gl_error::OpenGLError;
use std::ptr;
use std::fmt;
use std::ffi::CString;
use std::collections::HashMap;

pub const BASIC_VERTEX_SHADER: &'static str = include_str!("../shaders/basic_vert_shader.vs");
pub const BASIC_FRAGMENT_SHADER: &'static str = include_str!("../shaders/basic_frag_shader.fs");


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ShaderType {
    VertexShader,
    TessControlShader,
    TessEvaluationShader,
    GeometryShader,
    FragmentShader,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<u32> for &ShaderType {
    fn into(self) -> u32 {
        match self {
            ShaderType::VertexShader            => gl::VERTEX_SHADER,
            ShaderType::TessControlShader       => gl::TESS_CONTROL_SHADER,
            ShaderType::TessEvaluationShader    => gl::TESS_EVALUATION_SHADER,
            ShaderType::GeometryShader          => gl::GEOMETRY_SHADER,
            ShaderType::FragmentShader          => gl::FRAGMENT_SHADER,
        }
    } 
}

#[derive(Debug)]
pub struct Shader {
    sdr_type: ShaderType,
    id: u32,
    src: String,
}

impl Shader {
    pub fn new(gl_ctx: &gl::Gl, src: &str, shader_type: ShaderType) -> Shader {
        let sdr_id;
        unsafe {
            sdr_id = gl_ctx.CreateShader((&shader_type).into());
        }
        Shader {
            sdr_type: shader_type,
            id: sdr_id,
            src: src.to_owned(),
        }
    }

    pub fn compile_shader(&self, gl_ctx: &gl::Gl) -> Result<(), OpenGLError> {
        let src_str = CString::new(self.src.clone()).expect("Internal NULL detected. Shader failed to convert to C string.");
        unsafe {
            gl_ctx.ShaderSource(self.id, 1, &(src_str.as_ptr() as *const i8), ptr::null());
            gl_ctx.CompileShader(self.id);
            let mut result_code: i32 = 0;
            gl_ctx.GetShaderiv(self.id, gl::COMPILE_STATUS, (&mut result_code) as *mut i32);
            if result_code != (gl::TRUE as i32) {
                let mut character_output: [u8; 512] = [0; 512];
                gl_ctx.GetShaderInfoLog(self.id, 512, ptr::null_mut(), character_output.as_mut_ptr() as *mut i8);
                return Err(OpenGLError::CompileError(String::from_utf8_unchecked(character_output.to_vec())));
            }
        }
        Ok(())
    }

    pub fn delete_shader(self, gl_ctx: &gl::Gl) {
        unsafe {
            gl_ctx.DeleteShader(self.id)
        }
    }
}

#[derive(Debug)]

pub struct ShaderProgram {
    id: u32,
    loaded_phases: HashMap<ShaderType, Shader>,
}

impl ShaderProgram {
    pub fn new(gl_ctx: &gl::Gl) -> ShaderProgram {
        let prog_id;
        unsafe {
            prog_id = gl_ctx.CreateProgram();
        }
        ShaderProgram {
            id: prog_id,
            loaded_phases: HashMap::new(),
        }
    }

    // Determine the best method for having shaders in multiple places
    // Probably have a ShaderRef object that can only last as long as Shader
    // Since shaders should only be able to be deleted when they're all unlinked
    // And when we compile a shader we will unlink it before converting it to a CompiledShaderProgram.
    pub fn attach_shader(&mut self, gl_ctx: &gl::Gl, shader: Shader) -> Result<(), OpenGLError> {
        if self.loaded_phases.contains_key(&(&shader).sdr_type) {
            // We tried to attach an already attached shader to this program!
            return Err(OpenGLError::ProgramAlreadyContainedShader(shader.sdr_type));
        }

        unsafe {
            shader.compile_shader(gl_ctx)?;
            gl_ctx.AttachShader(self.id, shader.id);
        }

        self.loaded_phases.insert(shader.sdr_type, shader);
        Ok(())
    }
}

pub struct CompiledShaderProgram {
    id: u32,
}

impl CompiledShaderProgram {
    pub fn compile_shader(gl_ctx: &gl::Gl, prog: ShaderProgram) -> Result<CompiledShaderProgram, OpenGLError> {
        unsafe {
            gl_ctx.LinkProgram(prog.id);
            let mut result_code = 0;
            gl_ctx.GetShaderiv(prog.id, gl::LINK_STATUS, (&mut result_code) as *mut _);
            if result_code != 0 {
                let mut character_output: [u8; 512] = [0; 512];
                gl_ctx.GetShaderInfoLog(prog.id, 512, ptr::null_mut(), character_output.as_mut_ptr() as *mut i8);
                return Err(OpenGLError::LinkerError(prog, String::from_utf8_unchecked(character_output.to_vec())));
            }
            for (_,v) in prog.loaded_phases.iter() {
                gl_ctx.DetachShader(prog.id, v.id)
            }
            Ok(CompiledShaderProgram {
                id: prog.id
            })
        }
    }

    pub fn use_program(&self, gl_ctx: &gl::Gl) {
        unsafe {
            gl_ctx.UseProgram(self.id);
        }
    }

    pub fn unbind_program(&self, gl_ctx: &gl::Gl) {
        unsafe {
            gl_ctx.UseProgram(0);
        }
    }
}