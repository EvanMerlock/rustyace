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

mod types;
mod components;
mod utils;
mod asset_loading;
mod debug;

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
    debug::init_debug_context(&mut glfw);

    // TODO: Read from configuration file to set some window defaults!
    let (mut window, events) = glfw.create_window(300, 300, "RustyAce", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Migrated GL context to a ref-counted pointer inside all buffer/rendering structs.
    // This isn't as efficient as passing around references, and should eventually be migrated to lifetimes.
    let gl_context = gl::Gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl_context.Viewport(0,0,300,300);
    }

    let mut assets = asset_loading::AssetContainer::new("./assets", gl_context);
    debug::init_debug_functionality(assets.gl_ctx());

    //let obj_model = ObjModel::from_file(assets.gl_ctx(), "./assets/test/backpack.obj");


    // TODO: Develop a framebuffer container?
    // Not sure how we should be properly managing framebuffers tbh
    // esp. considering screen size can change... the framebuffer might need to realloc every time the screen size changes which is yikes
    let win_size = window.get_size();
    let mut single_pass_fbo = FrameBuffer::new(assets.gl_ctx());
    single_pass_fbo.bind(FrameBufferRDBehavior::RD);
    single_pass_fbo.attach_texture(win_size.0, win_size.1, 
        TexConfig::new(TextureType::Texture2D, InternalStorage::RGB, PixelDataFormat::RGB, PixelDataType::UnsignedByte), 
        FrameBufferAttachment::Color(0));
    single_pass_fbo.attach_renderbuffer(win_size.0, win_size.1, InternalStorage::Depth24Stencil8, FrameBufferAttachment::DepthStencil);
    single_pass_fbo.unbind();

    // TODO: develop an asset container
    // We shouldn't have to manually specify all of the assets the program uses in the main function
    // An asset container should be used to store it all
    let tex_config = TexConfig::new(
        TextureType::Texture2D, InternalStorage::RGB,
        PixelDataFormat::RGB, PixelDataType::UnsignedByte
    );
    assets.add_texture("texture1", "texture1.jpg", tex_config.clone())?;
    assets.add_texture("texture2", "texture2.png", tex_config.clone())?;

    let tex_config_cm = TexConfig::new(
        TextureType::TextureCubeMap, InternalStorage::RGB,
        PixelDataFormat::RGB, PixelDataType::UnsignedByte
    );
    assets.add_cubemap("skybox", "skybox", tex_config_cm)?;
    let assembled_shader = assets.add_program("shader_basic", "basic/tex_norm/vertex_tex_norm.vert", "basic/tex_norm/fragment_tex_norm.frag", None)?;
    
    assembled_shader.use_program();
    assembled_shader.assign_texture_to_unit("texture1", types::TextureUnit::Slot0);
    assembled_shader.assign_texture_to_unit("texture2", types::TextureUnit::Slot1);


    let screenspace_shader = assets.add_program("screenspace_shader", "frame/framebuffer.vert", "frame/framebuffer.frag", None)?;
    screenspace_shader.use_program();
    screenspace_shader.assign_texture_to_unit("screenTexture", types::TextureUnit::Slot0);

    let skybox_shader = assets.add_program("skybox_shader", "skybox/skybox.vert", "skybox/skybox.frag", None)?;
    skybox_shader.use_program();
    skybox_shader.assign_texture_to_unit("skybox", TextureUnit::Slot0);

    // TODO: Develop a model file format or use a pre-existing one (.obj comes to mind)
    // The only reason we would develop our own model file format is that this is designed to be a voxel engine;
    // However, using an existing model format would allow for some non-voxel objects to be implemented
    // Food for thought
    // Note that any voxel format would still need to specify extended surfaces; rendering a ton of individual cubes might be rough on the GPU
    // Although instanced rendering might be able to help reduce the issue
    // We could also implement both a voxel-model format and a normal model format, to make it easier to develop voxel models while also allowing model flexibility.
    let cube_model = Rc::new(ResidentModel::new(assets.gl_ctx(), &renderable::CUBE_VERTICES, &renderable::CUBE_INDICES, assembled_shader, |vao| {
        //TODO: is this the best way to have configurable attribute indices?
        // Is there a more elegant solution?
        // Or this this the most elegant solution?
        // We seem to be using callbacks at least twice, including this one.
        // This might be the best solution, although some form of interchange between shaders and VAO's would maybe be good (since shaders define what they accept)

        // Now that VAO's are associated with models, they can be configured on a per-model basis.
        // _Most_ models should load in with the same set of vertices (position, color?, texture, normal)
        // HOWEVER, there are special cases! Thus, ResidentModel should probably keep the VAO configuration, and file loaded models should automagically configure the VAO.
        // Esp. considering that OBJ files have the same vertex formatting! 

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
    }));

    let screenspace_quad = Rc::new(ResidentModel::new(assets.gl_ctx(), &renderable::QUAD_VERTICIES, &renderable::QUAD_INDICIES, screenspace_shader, |vao| {
        vao.configure_index(0, AttributeProperties::new(
            AttributeComponentSize::Two,
            GLType::Float, 
            false, 
            4, 
            0));

        vao.configure_index(1, AttributeProperties::new(
            AttributeComponentSize::Two, 
            GLType::Float, 
            false, 
            4, 
            2));
    }));

    let skybox_model = Rc::new(ResidentModel::new(assets.gl_ctx(), &renderable::CUBE_VERTICES, &renderable::CUBE_INDICES, skybox_shader, |vao| {
        vao.configure_index(
            0, 
            AttributeProperties::new(
                AttributeComponentSize::Three, 
                GLType::Float, 
                false, 
                8, 
                0));
    }));

    let mut camera = camera::Camera::new(
        glm::vec3(0.0, 0.0, 3.0), 
        glm::vec3(0.0, 1.0, 0.0), 
        glm::vec3(0.0, 0.0, -1.0),
        camera::PITCH, 
        camera::YAW, 
        camera::SENSITIVITY, 
        45.0
    );

    let cube_render = renderable::Renderable::new(assets.gl_ctx(), cube_model)?;
    let quad_render = renderable::Renderable::new(assets.gl_ctx(), screenspace_quad.clone())?;
    let skybox_render = renderable::Renderable::new(assets.gl_ctx(), skybox_model)?;

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

        let mut entry_context = EntryContext {
            dt: delta_t,
            first_mouse: &mut first_mouse,
            last_mouse_x: &mut last_mouse_x,
            last_mouse_y: &mut last_mouse_y,
            gl_context: assets.gl_ctx(),
            window: &mut window,
            camera: &mut camera,
        };

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut entry_context, event);
        }

        process_input(&mut entry_context);

        let (width, height) = window.get_size();
        let view_matrix = camera.generate_view_matrix();
        let projection_matrix = camera.generate_projection_matrix(width as f32, height as f32);

        // --- BEGIN RENDER PASS ---
        // todo: this will all be moved into a render system, but for now, we're leaving it as it is
        single_pass_fbo.bind(FrameBufferRDBehavior::RD);
        // We're now rendering inside the FBO
        unsafe {
            // TODO: move this into it's own function
            let gl_ctx = assets.gl_ctx();
            gl_ctx.Enable(gl::DEPTH_TEST);
            gl_ctx.ClearColor(0.0f32, 0.2f32, 0.0f32, 1.0f32);
            gl_ctx.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // TODO: Convert this to a real ECS system and implement physics.
        // Like _that's_ going to be easy.
        // Physics will need to take into account time-steps
        // and so will networking!
        // Multiple threads, either per-object or per-system (although networking should defintely be on its own thread. Packet consistency is important)
        // See if nphysics will work or if we'll need to do our own crude physics modeling
        cube_render.render(GLMode::Triangles, |shdr| {
            // This is a function that allows per-frame uniform setting. This will become important with transformations,
            // As this can be used to change the position of an object per-frame...
            // However, it could be wrapped in an optional member or perhaps another method to allow for rendering with shaders that do not have uniforms without passing in an empty closure
            assets.find_texture("texture1").expect("Failed to find texture").bind(TextureUnit::Slot0);
            assets.find_texture("texture2").expect("Failed to find texture").bind(TextureUnit::Slot1);

            let model = glm::rotate(&glm::Mat4::identity(), (glfw.get_time() as f32) * utils::radians(50.0), &glm::vec3(0.5, 1.0, 0.0));
            shdr.set_uniform("model", &model);
            shdr.set_uniform("view", &view_matrix);
            shdr.set_uniform("projection", &projection_matrix);
        })?;


        // -- render skybox here --
        unsafe {
            let gl_ctx = assets.gl_ctx();
            gl_ctx.DepthFunc(gl::LEQUAL);
        }

        skybox_render.render(GLMode::Triangles, |shdr| {
            assets.find_texture("skybox").expect("Failed to find texture").bind(TextureUnit::Slot0);

            let view = glm::mat3_to_mat4(&glm::mat4_to_mat3(&view_matrix));
            shdr.set_uniform("view", &view);
            shdr.set_uniform("projection", &projection_matrix);
        })?;

        unsafe {
            let gl_ctx = assets.gl_ctx();
            gl_ctx.DepthFunc(gl::LESS);
        }


        single_pass_fbo.unbind();
        // We're no longer rendering inside the FBO.
        unsafe {
            // TODO: move this into it's own function
            let gl_ctx = assets.gl_ctx();
            gl_ctx.Disable(gl::DEPTH_TEST);
            gl_ctx.ClearColor(1.0, 1.0, 1.0, 1.0);
            gl_ctx.Clear(gl::COLOR_BUFFER_BIT);
        }
        quad_render.render(GLMode::Triangles, |_| {
            // There are no uniforms for this!
            single_pass_fbo.get_texture(0).bind(TextureUnit::Slot0);
        })?;

        window.swap_buffers();

        // --- END RENDER PASS ---
    }


    Ok(())
}

struct EntryContext<'a> {
    dt: f32,
    first_mouse: &'a mut bool,
    last_mouse_x: &'a mut f32,
    last_mouse_y: &'a mut f32,
    gl_context: Rc<gl::Gl>,
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
    OpenGLError(OpenGLError),
    #[error("Asset Load Failed: {0}")]
    IOError(io::Error),
    #[error("Image Load Failed: {0}")]
    ImageError(image::ImageError),
    #[error("Bad Texture: {0}")]
    TextureError(TextureError),
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
}

impl From<OpenGLError> for RustyAceError {
    fn from(err: OpenGLError) -> Self {
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
            _ => RustyAceError::TextureError(err),
        }
    }
}