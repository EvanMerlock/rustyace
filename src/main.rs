#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(future_incompatible)]

use glfw::{Action, Context, Key};
use std::rc::Rc;
use std::io;
use image;
use thiserror::Error;
use types::*;


mod buffers;
mod renderable;
mod shaders;
mod gl_error;
mod textures;
mod types;

mod gl {
    use std::fmt;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl fmt::Debug for Gl {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "GL Context")
        }
    }
}

fn main() -> Result<(), RustyAceError> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // TODO: Read from configuration file to set some window defaults!
    let (mut window, events) = glfw.create_window(300, 300, "RustyAce", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Migrated GL context to a ref-counted pointer inside all buffer/rendering structs.
    // This isn't as efficient as passing around references, and should eventually be migrated to lifetimes.
    let gl_context = Rc::new(gl::Gl::load_with(|s| window.get_proc_address(s) as *const _));
    unsafe {
        gl_context.Viewport(0,0,300,300);
    }

    // TODO: develop an asset container
    // We shouldn't have to manually specify all of the assets the program uses in the main function
    // An asset container should be used to store it all
    let tex1 = textures::Texture::from_file(gl_context.clone(), "./textures/texture1.jpg")?;
    let tex2 = textures::Texture::from_file(gl_context.clone(), "./textures/texture2.png")?;
    let assembled_shader = Rc::new(shaders::CompiledShaderProgram::generate_program(gl_context.clone(), "./shaders/basic_vert_shader.vs", "./shaders/basic_frag_shader.fs", None)?);
    assembled_shader.use_program();
    assembled_shader.assign_texture_to_unit("texture1", &tex1, types::TextureUnit::Slot0);
    assembled_shader.assign_texture_to_unit("texture2", &tex2, types::TextureUnit::Slot1);

    // TODO: Develop a model file format or use a pre-existing one (.obj comes to mind)
    // The only reason we would develop our own model file format is that this is designed to be a voxel engine;
    // However, using an existing model format would allow for some non-voxel objects to be implemented
    // Food for thought
    // Note that any voxel format would still need to specify extended surfaces; rendering a ton of individual cubes might be rough on the GPU
    // Although instanced rendering might be able to help reduce the issue
    // We could also implement both a voxel-model format and a normal model format, to make it easier to develop voxel models while also allowing model flexibility.
    let tri_model = Rc::new(renderable::ResidentModel::new(&renderable::TRI_VERTICES, &renderable::TRI_INDICES, assembled_shader));

    let render = renderable::Renderable::new(gl_context.clone(), tri_model, |vao| {
        //TODO: is this the best way to have configurable attribute indices?
        // Is there a more elegant solution?
        // Or this this the most elegant solution?
        // We seem to be using callbacks at least twice, including this one.
        // This might be the best solution, although some form of interchange between shaders and VAO's would maybe be good (since shaders define what they accept)
        // Should a renderable have a 1-to-1 correspondence with a VAO?
        vao.configure_index(
            0, 
            buffers::AttributeProperties::new(
                AttributeComponentSize::Three, 
                GLType::Float, 
                false, 
                8, 
                0));

        vao.configure_index(1, buffers::AttributeProperties::new(
            AttributeComponentSize::Three, 
            GLType::Float, 
            false, 
            8, 
            3));

        vao.configure_index(2,
            buffers::AttributeProperties::new(
                AttributeComponentSize::Two,
                GLType::Float,
                false,
                8,
                6
            )
        );
    })?;

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    while !window.should_close() {

        unsafe {
            // TODO: move this into it's own function
            gl_context.ClearColor(0.0f32, 0.2f32, 0.0f32, 1.0f32);
            gl_context.Clear(gl::COLOR_BUFFER_BIT);
        }

        // TODO: Convert this to a real ECS system and implement physics.
        // Like _that's_ going to be easy.
        // Physics will need to take into account time-steps
        // See if nphysics will work or if we'll need to do our own crude physics modeling
        render.render(GLMode::Triangles, |shdr| {
            // This is a function that allows per-frame uniform setting. This will become important with transformations,
            // As this can be used to change the position of an object per-frame...
            // However, it could be wrapped in an optional member or perhaps another method to allow for rendering with shaders that do not have uniforms without passing in an empty closure
            shdr.set_uniform("transform", &(nalgebra::Matrix4::<f32>::new_scaling(1.0) * nalgebra::Matrix4::<f32>::from_scaled_axis(&nalgebra::Vector3::<f32>::new(0.0, 0.5, 1.0) * glfw.get_time() as f32)));
        })?;


        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&gl_context, &mut window, event);
        }
        window.swap_buffers();
    }


    Ok(())
}

fn handle_window_event(gl_context: &gl::Gl, window: &mut glfw::Window, event: glfw::WindowEvent) {
    // TODO: Enable mouse events and move camera
    // Should proc an event to tell the camera to move
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl_context.Viewport(0, 0, width, height);
            }
        }
        _ => {}
    }
}

#[derive(Error, Debug)]
pub enum RustyAceError {
    #[error("OpenGL Failed: {0}")]
    OpenGLError(gl_error::OpenGLError),
    #[error("Asset Load Failed: {0}")]
    IOError(io::Error),
    #[error("Image Load Failed: {0}")]
    ImageError(image::ImageError),
}

impl From<gl_error::OpenGLError> for RustyAceError {
    fn from(err: gl_error::OpenGLError) -> Self {
        RustyAceError::OpenGLError(err)
    }
}

impl From<io::Error> for RustyAceError {
    fn from(err: io::Error) -> Self {
        RustyAceError::IOError(err)
    }
}

impl From<textures::TextureError> for RustyAceError {
    fn from(err: textures::TextureError) -> Self {
        match err {
            textures::TextureError::ImageError(err_i) => RustyAceError::ImageError(err_i),
            textures::TextureError::OpenGLError(err_o) => RustyAceError::OpenGLError(err_o),
        }
    }
}