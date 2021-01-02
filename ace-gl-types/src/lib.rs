// TODO: move all of my types into this one module, one per file.
// This will keep them in one place for importing
#![allow(dead_code)]
#![deny(nonstandard_style)]
#![deny(rust_2018_idioms)]
#![deny(future_incompatible)]

pub mod gl {
    use std::fmt;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl fmt::Debug for Gl {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "GL Context")
        }
    }
}


// Modules
mod gl_tex_unit;
mod gl_draw_mode;
mod gl_type;
mod attribute_component_size;
mod gl_mode;
mod uniform_type;
mod texture;
mod buffers;
mod gl_error;
mod shaders;
mod typed_buffer;
mod model;

// Re-exports for convienence
pub use gl_tex_unit::*;
pub use gl_draw_mode::*;
pub use gl_type::*;
pub use attribute_component_size::*;
pub use gl_mode::*;
pub(crate) use uniform_type::*;
pub use self::texture::*;
pub use self::buffers::*;
pub use self::gl_error::*;
pub use self::shaders::*;
pub use self::typed_buffer::*;
pub use self::model::*;

pub use crate as types;