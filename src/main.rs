use glfw::{Action, Context, Key};
use std::rc::Rc;
use std::{io, mem};
use thiserror::Error;


mod buffers;
mod renderable;
mod shaders;
mod gl_error;

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

    let assembled_shader = Rc::new(shaders::CompiledShaderProgram::generate_program(gl_context.clone(), "./shaders/basic_vert_shader.vs", "./shaders/basic_frag_shader.fs", None)?);

    // TODO: Develop a model file format or use a pre-existing one
    // The only reason we would develop our own model file format is that this is designed to be a voxel engine;
    // However, using an existing model format would allow for some non-voxel objects to be implemented
    // Food for thought
    // We could also implement both a voxel-model format and a normal model format, to make it easier to develop voxel models while also allowing model flexibility.
    let tri_model = Rc::new(renderable::ResidentModel::new(&renderable::TRI_VERTICES, &renderable::TRI_INDICES, assembled_shader));

    let render = renderable::Renderable::new(gl_context.clone(), tri_model, |vao| {
        //TODO: is this the best way to have configurable attribute indices?
        // Is there a more elegant solution that involves tying them to the model?
        // Would tying a VAO to a model be poor practice? (seems so, as this would also tie shaders to that model specifically as well)
        // This might be the best solution, although some form of interchange between shaders and VAO's would maybe be good
        vao.configure_index(
            0, 
            buffers::AttributeProperties::new(
                buffers::AttributeComponentSize::Three, 
                buffers::GLType::Float, 
                false, 
                3 * mem::size_of::<f32>() as i32, 
                0));
    })?;

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    while !window.should_close() {

        unsafe {
            gl_context.ClearColor(0.0f32, 0.2f32, 0.0f32, 1.0f32);
            gl_context.Clear(gl::COLOR_BUFFER_BIT);
        }

        // TODO: Convert this to a real ECS system and implement physics.
        // Like _that's_ going to be easy.
        // Physics will need to take into account time-steps
        // See if nphysics will work or if we'll need to do our own crude physics modeling
        render.render(renderable::GLMode::Triangles)?;


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