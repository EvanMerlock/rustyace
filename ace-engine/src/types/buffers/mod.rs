mod ebo;
mod vao;
mod vbo;
mod framebuffer;

pub use self::ebo::*;
pub use self::vao::*;
pub use self::vbo::*;
pub use self::framebuffer::*;

// TODO:
// Buffer objects/shaders are GLOBAL STATE.
// We need a global lock for each type of buffer and shader programs, and then have buffers switch between a unbound and bound struct/state
// This allows us to prevent mistakes from binding buffers incorrectly (rebinding over a buffer before it's been dropped)