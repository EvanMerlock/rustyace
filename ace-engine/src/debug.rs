use crate::gl;
use glfw;
use std::rc::Rc;
use std::os::raw;
use std::ptr;
use std::ffi;

extern "system" fn debug_callback(source: raw::c_uint, gl_type: raw::c_uint, id: raw::c_uint, severity: raw::c_uint, _: raw::c_int, msg: *const raw::c_char, _: *mut raw::c_void) {

    // Filter out insignificant error/warning codes
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        return;
    }

    let mut buf = String::new();
    buf.push_str("---\n");
    unsafe {
        let msg_reg = ffi::CStr::from_ptr(msg).to_str().expect("failed to convert msg");
        buf.push_str(&format!("DEBUG MESSAGE {}: {}\n", id, msg_reg));
    }

    match source {
        gl::DEBUG_SOURCE_API                => { buf.push_str(&format!("Source: API\n")) },
        gl::DEBUG_SOURCE_WINDOW_SYSTEM      => { buf.push_str(&format!("Source: Window System\n")) },
        gl::DEBUG_SOURCE_SHADER_COMPILER    => { buf.push_str(&format!("Source: Shader Compiler\n")) },
        gl::DEBUG_SOURCE_THIRD_PARTY        => { buf.push_str(&format!("Source: Third Party\n")) },
        gl::DEBUG_SOURCE_APPLICATION        => { buf.push_str(&format!("Source: Application\n")) },
        gl::DEBUG_SOURCE_OTHER              => { buf.push_str(&format!("Source: Other\n")) },
        _ => {},
    }

    match gl_type {
        gl::DEBUG_TYPE_ERROR                => { buf.push_str(&format!("Type: Error\n")) },
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR  => { buf.push_str(&format!("Type: Deprecated Behavior\n")) },
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR   => { buf.push_str(&format!("Type: Undefined Behavior\n")) },
        gl::DEBUG_TYPE_PORTABILITY          => { buf.push_str(&format!("Type: Portability\n")) },
        gl::DEBUG_TYPE_PERFORMANCE          => { buf.push_str(&format!("Type: Performance\n")) },
        gl::DEBUG_TYPE_MARKER               => { buf.push_str(&format!("Type: Marker\n")) },
        gl::DEBUG_TYPE_PUSH_GROUP           => { buf.push_str(&format!("Type: Push Group\n")) },
        gl::DEBUG_TYPE_POP_GROUP            => { buf.push_str(&format!("Type: Pop Group\n")) },
        gl::DEBUG_TYPE_OTHER                => { buf.push_str(&format!("Type: Other\n")) },
        _ => {}
    }

    match severity {
        gl::DEBUG_SEVERITY_HIGH         => { buf.push_str(&format!("Severity: High\n")) },
        gl::DEBUG_SEVERITY_MEDIUM       => { buf.push_str(&format!("Severity: Medium\n")) },
        gl::DEBUG_SEVERITY_LOW          => { buf.push_str(&format!("Severity: Low\n")) },
        gl::DEBUG_SEVERITY_NOTIFICATION => { buf.push_str(&format!("Severity: Notification\n")) },
        _ => {}
    }

    println!("{}", buf);


}

pub fn init_debug_context(context: &mut glfw::Glfw) {
    context.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
}

pub fn init_debug_functionality(gl_ctx: Rc<gl::Gl>) {
    let mut gl_flags: i32 = 0;
    unsafe {
        gl_ctx.GetIntegerv(gl::CONTEXT_FLAGS, &mut gl_flags as *mut _);
    }

    if ((gl_flags as u32) & gl::CONTEXT_FLAG_DEBUG_BIT) != 0 {
        // initialize debug output
        unsafe {
            gl_ctx.Enable(gl::DEBUG_OUTPUT);
            gl_ctx.Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl_ctx.DebugMessageCallback(Some(debug_callback), ptr::null());
            gl_ctx.DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, ptr::null(), gl::TRUE);
        }
    }
}