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
use std::borrow::Borrow;
use nalgebra;

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

    pub(crate) fn set_uniform<T: UniformType>(&self, name: &str, uniform: &T) {
        let loc: i32;
        unsafe {
            let c_str = CString::new(name).expect("Internal NULL detected. Uniform location failed to convert to valid CString");
            loc = self.gl_ctx.GetUniformLocation(self.id, c_str.as_ptr());
        }
        uniform.assign_to_current_program(self.gl_ctx.as_ref(), loc);
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

pub(crate) trait UniformType {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32);
}

impl UniformType for f32 {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1f(loc, *self);
        }
    }
}

impl UniformType for nalgebra::Vector2<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2f(loc, self[0], self[1]);
        }
    }
}

impl UniformType for nalgebra::Vector3<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3f(loc, self[0], self[1], self[2]);
        }
    }
}

impl UniformType for nalgebra::Vector4<f32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4f(loc, self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformType for &[f32] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1fv(loc, self.len() as i32, self.as_ptr());
        }
    }
}

impl UniformType for &[nalgebra::Vector2<f32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2fv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector3<f32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3fv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector4<f32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4fv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

// ---

impl UniformType for i32 {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1i(loc, *self);
        }
    }
}

impl UniformType for nalgebra::Vector2<i32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2i(loc, self[0], self[1]);
        }
    }
}

impl UniformType for nalgebra::Vector3<i32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3i(loc, self[0], self[1], self[2]);
        }
    }
}

impl UniformType for nalgebra::Vector4<i32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4i(loc, self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformType for &[i32] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1iv(loc, self.len() as i32, self.as_ptr());
        }
    }
}

impl UniformType for &[nalgebra::Vector2<i32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2iv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector3<i32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3iv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector4<i32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4iv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

// ---

impl UniformType for u32 {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1ui(loc, *self);
        }
    }
}

impl UniformType for nalgebra::Vector2<u32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2ui(loc, self[0], self[1]);
        }
    }
}

impl UniformType for nalgebra::Vector3<u32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3ui(loc, self[0], self[1], self[2]);
        }
    }
}

impl UniformType for nalgebra::Vector4<u32> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4ui(loc, self[0], self[1], self[2], self[3]);
        }
    }
}

impl UniformType for &[u32] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1uiv(loc, self.len() as i32, self.as_ptr());
        }
    }
}

impl UniformType for &[nalgebra::Vector2<u32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2uiv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector3<u32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3uiv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

impl UniformType for &[nalgebra::Vector4<u32>] {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4uiv(loc, self.len() as i32, self.as_ptr() as *const _);
        }
    }
}

// ---

impl UniformType for bool {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform1i(loc, (*self) as i32);
        }
    }
}

impl UniformType for nalgebra::Vector2<bool> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform2i(loc, self[0] as i32, self[1] as i32);
        }
    }
}

impl UniformType for nalgebra::Vector3<bool> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform3i(loc, self[0] as i32, self[1] as i32, self[2] as i32);
        }
    }
}

impl UniformType for nalgebra::Vector4<bool> {
    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        unsafe {
            gl_ctx.Uniform4i(loc, self[0] as i32, self[1] as i32, self[2] as i32, self[3] as i32);
        }
    }
}