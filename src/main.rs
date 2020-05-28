#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(future_incompatible)]

use glfw::{Action, Context, Key};
use std::rc::Rc;
use std::io;
use image;
use thiserror::Error;
use types::*;
use components::*;
use nalgebra_glm as glm;

mod shaders;
mod gl_error;
mod types;
mod components;
mod utils;

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

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Migrated GL context to a ref-counted pointer inside all buffer/rendering structs.
    // This isn't as efficient as passing around references, and should eventually be migrated to lifetimes.
    let gl_context = Rc::new(gl::Gl::load_with(|s| window.get_proc_address(s) as *const _));
    unsafe {
        gl_context.Viewport(0,0,300,300);
        gl_context.Enable(gl::DEPTH_TEST);
    }

    // TODO: develop an asset container
    // We shouldn't have to manually specify all of the assets the program uses in the main function
    // An asset container should be used to store it all
    let tex1 = Texture::from_file(gl_context.clone(), "./textures/texture1.jpg")?;
    let tex2 = Texture::from_file(gl_context.clone(), "./textures/texture2.png")?;
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
    let tri_model = Rc::new(renderable::ResidentModel::new(&renderable::CUBE_VERTICES, &renderable::CUBE_INDICES, assembled_shader));

    let mut camera = camera::Camera::new(
        glm::vec3(0.0, 0.0, 3.0), 
        glm::vec3(0.0, 1.0, 0.0), 
        glm::vec3(0.0, 0.0, -1.0),
        camera::PITCH, 
        camera::YAW, 
        camera::SENSITIVITY, 
        45.0
    );

    let render = renderable::Renderable::new(gl_context.clone(), tri_model, |vao| {
        //TODO: is this the best way to have configurable attribute indices?
        // Is there a more elegant solution?
        // Or this this the most elegant solution?
        // We seem to be using callbacks at least twice, including this one.
        // This might be the best solution, although some form of interchange between shaders and VAO's would maybe be good (since shaders define what they accept)
        // Should a renderable have a 1-to-1 correspondence with a VAO?
        vao.configure_index(
            0, 
            AttributeProperties::new(
                AttributeComponentSize::Three, 
                GLType::Float, 
                false, 
                8, 
                0));

        vao.configure_index(1, AttributeProperties::new(
            AttributeComponentSize::Three, 
            GLType::Float, 
            false, 
            8, 
            3));

        vao.configure_index(2,
            AttributeProperties::new(
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
    window.set_cursor_pos_polling(true);
    window.make_current();

    let mut last_frame: f32 = 0.0;
    let mut delta_t: f32;

    let mut first_mouse = true;
    let mut last_mouse_x = 0.0;
    let mut last_mouse_y = 0.0;

    while !window.should_close() {

        let current_frame = glfw.get_time() as f32;
        delta_t = current_frame - last_frame;
        last_frame = current_frame;        

        unsafe {
            // TODO: move this into it's own function
            gl_context.ClearColor(0.0f32, 0.2f32, 0.0f32, 1.0f32);
            gl_context.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let mut entry_context = EntryContext {
            dt: delta_t,
            first_mouse: &mut first_mouse,
            last_mouse_x: &mut last_mouse_x,
            last_mouse_y: &mut last_mouse_y,
            gl_context: &gl_context,
            window: &mut window,
            camera: &mut camera,
        };

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut entry_context, event);
        }

        process_input(&mut entry_context);

        // TODO: Convert this to a real ECS system and implement physics.
        // Like _that's_ going to be easy.
        // Physics will need to take into account time-steps
        // See if nphysics will work or if we'll need to do our own crude physics modeling
        render.render(GLMode::Triangles, |shdr| {
            // This is a function that allows per-frame uniform setting. This will become important with transformations,
            // As this can be used to change the position of an object per-frame...
            // However, it could be wrapped in an optional member or perhaps another method to allow for rendering with shaders that do not have uniforms without passing in an empty closure
            let model = glm::rotate(&glm::Mat4::identity(), (glfw.get_time() as f32) * utils::radians(50.0), &glm::vec3(0.5, 1.0, 0.0));
            shdr.set_uniform("model", &model);
            shdr.set_uniform("view", &camera.generate_view_matrix());
            let (width, height) = window.get_size();
            shdr.set_uniform("projection", &camera.generate_projection_matrix(width as f32, height as f32));
        })?;

        window.swap_buffers();
    }


    Ok(())
}

struct EntryContext<'a> {
    dt: f32,
    first_mouse: &'a mut bool,
    last_mouse_x: &'a mut f32,
    last_mouse_y: &'a mut f32,
    gl_context: &'a gl::Gl,
    window: &'a mut glfw::Window,
    camera: &'a mut camera::Camera,
}

fn handle_window_event(ctx: &mut EntryContext<'_>, event: glfw::WindowEvent) {
    // TODO: Enable mouse events and move camera
    // Should proc an event to tell the camera to move
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            ctx.window.set_should_close(true)
        },
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                ctx.gl_context.Viewport(0, 0, width, height);
            }
        },
        glfw::WindowEvent::CursorPos(x, y) => {

            let x = x as f32;
            let y = y as f32;

            if *ctx.first_mouse {
                *ctx.first_mouse = false;
                *ctx.last_mouse_x = x;
                *ctx.last_mouse_y = y;
            }

            let x_offset = x - *ctx.last_mouse_x;
            let y_offset = *ctx.last_mouse_y - y;

            *ctx.last_mouse_x = x;
            *ctx.last_mouse_y = y;

            ctx.camera.process_mouse_input(x_offset, y_offset, true);
        },
        _ => {}
    }
}

fn process_input(ctx: &mut EntryContext<'_>) {
    if ctx.window.get_key(Key::W) == Action::Press {
        ctx.camera.process_movement(camera::CameraMovement::Fwd, ctx.dt);
    }
    if ctx.window.get_key(Key::A) == Action::Press {
        ctx.camera.process_movement(camera::CameraMovement::Left, ctx.dt);
    }
    if ctx.window.get_key(Key::S) == Action::Press {
        ctx.camera.process_movement(camera::CameraMovement::Bwd, ctx.dt);
    }
    if ctx.window.get_key(Key::D) == Action::Press {
        ctx.camera.process_movement(camera::CameraMovement::Right, ctx.dt);
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

impl From<TextureError> for RustyAceError {
    fn from(err: TextureError) -> Self {
        match err {
            TextureError::ImageError(err_i) => RustyAceError::ImageError(err_i),
            TextureError::OpenGLError(err_o) => RustyAceError::OpenGLError(err_o),
        }
    }
}