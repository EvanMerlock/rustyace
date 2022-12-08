use std::collections::HashMap;
use std::rc::Rc;
use crate::types::*;
use ace_gl_types::gl;
use std::path::{Path, PathBuf};

pub struct AssetContainer {
    asset_root: PathBuf,
    // How in the hell do we free unused data?
    // Implement a GC? ;)
    gl_context: Rc<gl::Gl>,
    models: HashMap<String, Rc<dyn Model>>,
    materials: HashMap<String, Rc<Material>>,
    // is this necessary?
    // potentially if we wish to re-use shaders to recompile new programs
    // look into if major games keep shaders resident in memory
    // they shouldn't take up too much space, but you never know...
    //shaders: HashMap<String, Rc<Shader>>,
    shader_programs: HashMap<String, Rc<CompiledShaderProgram>>,
    textures: HashMap<String, Rc<Texture>>,

}

impl AssetContainer {
    pub fn new<S: AsRef<Path>>(asset_container_location: S, gl_context: gl::Gl) -> AssetContainer {
        AssetContainer {
            asset_root: asset_container_location.as_ref().to_path_buf(),
            gl_context: Rc::new(gl_context),
            models: HashMap::new(),
            materials: HashMap::new(),
            shader_programs: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn gl_ctx(&self) -> Rc<gl::Gl> {
        self.gl_context.clone()
    }

    pub fn add_program<S: AsRef<Path>, V: ToString>(&mut self, name: V, vertex_name: S, fragment_name: S, geometry_name: Option<S>) -> Result<Rc<CompiledShaderProgram>, ShaderCompileError> {
        // Initialize new shader program to attach shaders to
        let mut shdr_prog = ShaderProgram::new(self.gl_ctx());

        // Generate the asset paths
        let mut vs_path = self.asset_root.clone();
        vs_path.push("shaders");
        vs_path.push(vertex_name);

        let mut fs_path = self.asset_root.clone();
        fs_path.push("shaders");
        fs_path.push(fragment_name);

        // Build and compile the vertex shader
        let vs_shdr = Shader::from_path(self.gl_ctx().clone(), vs_path, ShaderType::VertexShader)?;
        vs_shdr.compile_shader()?;

        // Build and compile the fragment shader
        let fs_shdr = Shader::from_path(self.gl_ctx().clone(), fs_path, ShaderType::FragmentShader)?;
        fs_shdr.compile_shader()?;

        // Attach the vertex and fragment shader
        shdr_prog.attach_shader(&vs_shdr)?;
        shdr_prog.attach_shader(&fs_shdr)?;

        match geometry_name {
            Some(geo_internal_name) => {
                let mut gs_loc = self.asset_root.clone();
                gs_loc.push("shaders");
                gs_loc.push(geo_internal_name);
                let gs_shdr = Shader::from_path(self.gl_ctx().clone(), gs_loc, ShaderType::GeometryShader)?;
                shdr_prog.attach_shader(&gs_shdr)?;
                let csp = CompiledShaderProgram::compile_shader(self.gl_ctx().clone(), shdr_prog).map_err(|(err, _)| err)?;
                let csp_rc = Rc::new(csp);
                self.shader_programs.insert(name.to_string(), csp_rc.clone());
                Ok(csp_rc)
            },
            None => {
                let csp = CompiledShaderProgram::compile_shader(self.gl_ctx().clone(), shdr_prog).map_err(|(err, _)| err)?;
                let csp_rc = Rc::new(csp);
                self.shader_programs.insert(name.to_string(), csp_rc.clone());
                Ok(csp_rc)
            },
        }
    }

    pub fn find_program(&self, name: &str) -> Option<Rc<CompiledShaderProgram>> {
        self.shader_programs.get(name).map(|csp| csp.clone())
    }

    pub fn add_texture<S: AsRef<Path>, V: ToString>(&mut self, name: V, texture_name: S, texture_cfg: TexConfig) -> Result<Rc<Texture>, TextureError> {
        // Generate the asset path
        let mut tex_path = self.asset_root.clone();
        tex_path.push("textures");
        tex_path.push(texture_name);

        let new_tex = Rc::new(Texture::from_file(self.gl_ctx(), tex_path, texture_cfg)?);

        self.textures.insert(name.to_string(), new_tex.clone());
        Ok(new_tex)
    }

    pub fn add_cubemap<S: AsRef<Path>, V: ToString>(&mut self, name: V, cm_location: S, texture_cfg: TexConfig) -> Result<Rc<Texture>, TextureError> {
        let mut tex_path = self.asset_root.clone();
        tex_path.push("textures");
        tex_path.push(cm_location.as_ref());

        let new_tex = Rc::new(Texture::cubemap_from_files(self.gl_ctx(), CubemapPaths::from_directory(tex_path)?, texture_cfg)?);

        self.textures.insert(name.to_string(), new_tex.clone());
        Ok(new_tex)
    }

    pub fn find_texture(&self, name: &str) -> Option<Rc<Texture>> {
        self.textures.get(name).map(|tex| tex.clone())
    }

    pub fn add_material(&mut self, mat: Material) {
        self.materials.insert(mat.name.clone(), Rc::new(mat));
    }
}