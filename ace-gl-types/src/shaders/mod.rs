use crate::gl;
use std::rc::Rc;
use std::ptr;
use std::fmt;
use std::ffi::CString;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::path::Path;
use crate::types::*;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ShaderType {
    VertexShader,
    TessControlShader,
    TessEvaluationShader,
    GeometryShader,
    FragmentShader,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    gl_ctx: Rc<gl::Gl>,
    sdr_type: ShaderType,
    id: u32,
    src: String,
}

impl Shader {
    pub fn new(gl_ctx: Rc<gl::Gl>, src: &str, shader_type: ShaderType) -> Shader {
        Shader::new_from_string(gl_ctx, src.to_owned(), shader_type)
    }

    fn new_from_string(gl_ctx: Rc<gl::Gl>, src: String, shader_type: ShaderType) -> Shader {
        let sdr_id;
        unsafe {
            sdr_id = gl_ctx.CreateShader((&shader_type).into());
        }
        Shader {
            gl_ctx: gl_ctx,
            sdr_type: shader_type,
            id: sdr_id,
            src: src,
        }
    }

    pub fn from_path<S: AsRef<Path>>(gl_ctx: Rc<gl::Gl>, loc: S, shader_type: ShaderType) -> io::Result<Shader> {
        let file = fs::File::open(loc)?;
        let md = file.metadata()?;
        let buffered = io::BufReader::new(file);
        let mut src = String::with_capacity(md.len() as usize);
        for line_res in buffered.lines() {
            let mut line = line_res?;
            line.push('\n');
            src.push_str(&line);
        }

        Ok(Shader::new(gl_ctx, &src, shader_type))

    }

    pub fn compile_shader(&self) -> Result<(), OpenGLError> {
        let src_str = CString::new(self.src.clone()).expect("Internal NULL detected. Shader failed to convert to C string.");
        unsafe {
            self.gl_ctx.ShaderSource(self.id, 1, &(src_str.as_ptr() as *const i8), ptr::null());
            self.gl_ctx.CompileShader(self.id);
            let mut result_code: i32 = 0;
            self.gl_ctx.GetShaderiv(self.id, gl::COMPILE_STATUS, (&mut result_code) as *mut i32);
            if result_code != (gl::TRUE as i32) {
                let character_output = Vec::with_capacity(512);
                let c_str = CString::new(character_output).expect("Failed to initialize blank CString");
                let c_ptr = c_str.into_raw();
                self.gl_ctx.GetShaderInfoLog(self.id, 512, ptr::null_mut(), c_ptr);
                let c_str = CString::from_raw(c_ptr);
                return Err(OpenGLError::CompileError(c_str.to_str().expect("Failed to convert from CString").to_owned()));
            }
        }
        Ok(())
    }

    pub fn delete_shader(self) {
        unsafe {
            self.gl_ctx.DeleteShader(self.id)
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl_ctx.DeleteShader(self.id)
        }
    }
}

#[derive(Debug)]
pub struct ShaderProgram<'a> {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
    loaded_phases: HashMap<ShaderType, &'a Shader>,
}

impl<'a> ShaderProgram<'a> {
    pub fn new(gl_ctx: Rc<gl::Gl>) -> ShaderProgram<'a> {
        let prog_id;
        unsafe {
            prog_id = gl_ctx.CreateProgram();
        }
        ShaderProgram {
            gl_ctx: gl_ctx,
            id: prog_id,
            loaded_phases: HashMap::new(),
        }
    }

    pub fn attach_shader(&mut self, shader: &'a Shader) -> Result<(), OpenGLError> {
        if self.loaded_phases.contains_key(&(&shader).sdr_type) {
            // We tried to attach an already attached shader to this program!
            return Err(OpenGLError::ProgramAlreadyContainedShader(shader.sdr_type));
        }

        shader.compile_shader()?;
        unsafe {
            self.gl_ctx.AttachShader(self.id, shader.id);
        }

        self.loaded_phases.insert(shader.sdr_type, shader);
        Ok(())
    }
}

pub struct CompiledShaderProgram {
    gl_ctx: Rc<gl::Gl>,
    id: u32,
    // there needs to be something here to restore texture state, since glBindTexture overwrites what's being bound in which slot.
    // most likely a hashmap, but how do we store texture information?
    // Rc's would likely work, but that's a lot more rc pointers to deal with.
}

impl CompiledShaderProgram {
    pub fn compile_shader(gl_ctx: Rc<gl::Gl>, prog: ShaderProgram<'_>) -> Result<CompiledShaderProgram, (OpenGLError, ShaderProgram<'_>)> {
        unsafe {
            gl_ctx.LinkProgram(prog.id);
            let mut result_code = 0;
            gl_ctx.GetProgramiv(prog.id, gl::LINK_STATUS, (&mut result_code) as *mut _);
            if result_code != (gl::TRUE as i32) {
                let character_output = Vec::with_capacity(512);
                let c_str = CString::new(character_output).expect("Failed to initialize blank CString");
                let c_ptr = c_str.into_raw();
                gl_ctx.GetProgramInfoLog(prog.id, 512, ptr::null_mut(), c_ptr);
                let c_str = CString::from_raw(c_ptr);
                return Err((OpenGLError::LinkerError(c_str.to_str().expect("Failed to convert from CString").to_owned()), prog));
            }
            for (_,v) in prog.loaded_phases.iter() {
                gl_ctx.DetachShader(prog.id, v.id)
            }
            Ok(CompiledShaderProgram {
                gl_ctx: gl_ctx,
                id: prog.id
            })
        }
    }

    pub fn generate_program<S: AsRef<Path>>(gl_ctx: Rc<gl::Gl>, vs_path: S, fs_path: S, gs_path: Option<S>) -> Result<CompiledShaderProgram, ShaderCompileError> {
        let mut shdr_prog = ShaderProgram::new(gl_ctx.clone());
        let vs_shdr = Shader::from_path(gl_ctx.clone(), vs_path, ShaderType::VertexShader)?;
        vs_shdr.compile_shader()?;
        let fs_shdr = Shader::from_path(gl_ctx.clone(), fs_path, ShaderType::FragmentShader)?;
        fs_shdr.compile_shader()?;
        shdr_prog.attach_shader(&vs_shdr)?;
        shdr_prog.attach_shader(&fs_shdr)?;

        match gs_path {
            Some(gs_loc) => {
                let gs_shdr = Shader::from_path(gl_ctx.clone(), gs_loc, ShaderType::GeometryShader)?;
                shdr_prog.attach_shader(&gs_shdr)?;
                Ok(CompiledShaderProgram::compile_shader(gl_ctx.clone(), shdr_prog).map_err(|(err, _)| err)?)
            },
            None => {
                Ok(CompiledShaderProgram::compile_shader(gl_ctx.clone(), shdr_prog).map_err(|(err, _)| err)?)
            },
        }


    }

    pub fn set_uniform<T: UniformType>(&self, name: &str, uniform: &T) {
        let loc: i32;
        unsafe {
            let c_str = CString::new(name).expect("Internal NULL detected. Uniform location failed to convert to valid CString");
            loc = self.gl_ctx.GetUniformLocation(self.id, c_str.as_ptr());
        }
        uniform.assign_to_current_program(self.gl_ctx.as_ref(), loc);
    }

    pub fn assign_texture_to_unit(&self, name: &str, tex_unit: TextureUnit) {
        self.set_uniform(name, &tex_unit)
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl_ctx.UseProgram(self.id);
        }
    }

    pub fn unbind_program(&self) {
        unsafe {
            self.gl_ctx.UseProgram(0);
        }
    }
}

#[derive(Error, Debug)]
pub enum ShaderCompileError {
    #[error("OpenGL Error: {0}")]
    OpenGLError(OpenGLError),
    #[error("IO Error: {0}")]
    IOError(io::Error),
}

impl From<io::Error> for ShaderCompileError {
    fn from(err: io::Error) -> ShaderCompileError {
        ShaderCompileError::IOError(err)
    }
}

impl From<OpenGLError> for ShaderCompileError {
    fn from(err: OpenGLError) -> ShaderCompileError {
        ShaderCompileError::OpenGLError(err)
    }
}