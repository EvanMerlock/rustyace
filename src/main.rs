use glfw::{Action, Context, Key};
use std::rc::Rc;
use std::mem;

mod buffers;
mod renderable;
mod shaders;
mod gl_error;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() -> Result<(), gl_error::OpenGLError> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // TODO: Read from configuration file to set some window defaults!
    let (mut window, events) = glfw.create_window(300, 300, "RustyAce", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // TODO: The GL context should be stored within all of the structures that need frequent access to it and cannot outlive the GL context.
    // Right now, you could in theory make a second window and reuse items from the first GL context in the second one, which is _really_ bad and wrong.
    // Lifetimes should be utilized to prevent this issue.
    let gl_context = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl_context.Viewport(0,0,300,300);
    }

    // TODO: Is there a better way of doing this? Probably. Develop that method out further.
    // Perhaps a function to read multiple shaders from files?
    // That would probably be best as a method on CompiledShaderProgram, having it just take in the files and produce a CSP.
    // That would allow for the skipping of these steps unless one needs them.
    let vert_shdr = shaders::Shader::new(&gl_context, shaders::BASIC_VERTEX_SHADER, shaders::ShaderType::VertexShader);
    let frag_shdr = shaders::Shader::new(&gl_context, shaders::BASIC_FRAGMENT_SHADER, shaders::ShaderType::FragmentShader);

    let mut shdr_prog = shaders::ShaderProgram::new(&gl_context);
    shdr_prog.attach_shader(&gl_context, vert_shdr)?;
    shdr_prog.attach_shader(&gl_context, frag_shdr)?;

    let assembled_shader = Rc::new(shaders::CompiledShaderProgram::compile_shader(&gl_context, shdr_prog)?);

    // TODO: Develop a model file format or use a pre-existing one
    // The only reason we would develop our own model file format is that this is designed to be a voxel engine;
    // However, using an existing model format would allow for some non-voxel objects to be implemented
    // Food for thought
    // We could also implement both a voxel-model format and a normal model format, to make it easier to develop voxel models while also allowing model flexibility.
    let tri_model = Rc::new(renderable::ResidentModel::new(&renderable::TRI_VERTICES, &renderable::TRI_INDICES, assembled_shader));

    let render = renderable::Renderable::new(&gl_context, tri_model, |ctx, vao| {
        //TODO: is this the best way to have configurable attribute indices?
        // Is there a more elegant solution that involves tying them to the model?
        // Would tying a VAO to a model be poor practice? (seems so, as this would also tie shaders to that model specifically as well)
        // This might be the best solution, although some form of interchange between shaders and VAO's would maybe be good
        vao.configure_index(
            ctx, 0, 
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
        render.render(&gl_context, renderable::GLMode::Triangles)?;


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