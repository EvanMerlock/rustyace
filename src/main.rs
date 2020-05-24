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

    let (mut window, events) = glfw.create_window(300, 300, "RustyAce", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let gl_context = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl_context.Viewport(0,0,300,300);
    }

    let vert_shdr = shaders::Shader::new(&gl_context, shaders::BASIC_VERTEX_SHADER, shaders::ShaderType::VertexShader);
    let frag_shdr = shaders::Shader::new(&gl_context, shaders::BASIC_FRAGMENT_SHADER, shaders::ShaderType::FragmentShader);

    let mut shdr_prog = shaders::ShaderProgram::new(&gl_context);
    shdr_prog.attach_shader(&gl_context, vert_shdr)?;
    shdr_prog.attach_shader(&gl_context, frag_shdr)?;

    let assembled_shader = Rc::new(shaders::CompiledShaderProgram::compile_shader(&gl_context, shdr_prog)?);

    let tri_model = Rc::new(renderable::ResidentModel::new(&renderable::TRI_VERTICES, &renderable::TRI_INDICES, assembled_shader));

    let render = renderable::Renderable::new(&gl_context, tri_model, |ctx, vao| {
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