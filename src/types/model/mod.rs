use crate::types::*;
use std::rc::Rc;

mod memory_model;
pub use self::memory_model::*;

// TODO: Figure out if we need to split models into meshes (we probably do)
// And the best way to communicate data to the GPU.
// With nalgebra, we might be able to augment matricies by row (since matricies are column-major) in order to add more information
// So then we could have separate color/lighting/texture matricies
pub trait Model {
    fn get_vertices(&self)          -> &Vec<f32>;
    fn get_indices(&self)           -> &Vec<u32>;
    fn get_shader(&self)            -> &Rc<CompiledShaderProgram>;
    fn get_vert_array_obj(&self)    -> &VertexArrayObj;
    fn get_vert_buffer_obj(&self)   -> &VertexBufferObj;
    fn get_elem_array_obj(&self)    -> &ElementArrayObj;
}