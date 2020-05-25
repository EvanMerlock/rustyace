use super::gl;
use super::gl_error::OpenGLError;
use super::RustyAceError;
use std::rc::Rc;
use std::ptr;
use std::fmt;
use std::ffi::CString;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::path::Path;

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
                let mut character_output: [u8; 512] = [0; 512];
                self.gl_ctx.GetShaderInfoLog(self.id, 512, ptr::null_mut(), character_output.as_mut_ptr() as *mut i8);
                return Err(OpenGLError::CompileError(String::from_utf8_unchecked(character_output.to_vec())));
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
}

impl CompiledShaderProgram {
    pub fn compile_shader(gl_ctx: Rc<gl::Gl>, prog: ShaderProgram) -> Result<CompiledShaderProgram, (OpenGLError, ShaderProgram)> {
        unsafe {
            gl_ctx.LinkProgram(prog.id);
            let mut result_code = 0;
            gl_ctx.GetShaderiv(prog.id, gl::LINK_STATUS, (&mut result_code) as *mut _);
            if result_code != 0 {
                let mut character_output: [u8; 512] = [0; 512];
                gl_ctx.GetShaderInfoLog(prog.id, 512, ptr::null_mut(), character_output.as_mut_ptr() as *mut i8);
                return Err((OpenGLError::LinkerError(String::from_utf8_unchecked(character_output.to_vec())), prog));
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

    pub fn generate_program<S: AsRef<Path>>(gl_ctx: Rc<gl::Gl>, vs_path: S, fs_path: S, gs_path: Option<S>) -> Result<CompiledShaderProgram, RustyAceError> {
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