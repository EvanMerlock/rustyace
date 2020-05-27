use crate::gl;
use crate::types::UniformType;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum TextureUnit {
    Slot0 = gl::TEXTURE0 as isize,
    Slot1 = gl::TEXTURE1 as isize,
    Slot2 = gl::TEXTURE2 as isize,
    Slot3 = gl::TEXTURE3 as isize,
    Slot4 = gl::TEXTURE4 as isize,
    Slot5 = gl::TEXTURE5 as isize,
    Slot6 = gl::TEXTURE6 as isize,
    Slot7 = gl::TEXTURE7 as isize,
    Slot8 = gl::TEXTURE8 as isize,
    Slot9 = gl::TEXTURE9 as isize,
    Slot10 = gl::TEXTURE10 as isize,
    Slot11 = gl::TEXTURE11 as isize,
    Slot12 = gl::TEXTURE12 as isize,
    Slot13 = gl::TEXTURE13 as isize,
    Slot14 = gl::TEXTURE14 as isize,
    Slot15 = gl::TEXTURE15 as isize,
}

impl UniformType for TextureUnit {

    fn assign_to_current_program(&self, gl_ctx: &gl::Gl, loc: i32) {
        let uni_index = match self {
            TextureUnit::Slot0 => 0,
            TextureUnit::Slot1 => 1,
            TextureUnit::Slot2 => 2,
            TextureUnit::Slot3 => 3,
            TextureUnit::Slot4 => 4,
            TextureUnit::Slot5 => 5,
            TextureUnit::Slot6 => 6,
            TextureUnit::Slot7 => 7,
            TextureUnit::Slot8 => 8,
            TextureUnit::Slot9 => 9,
            TextureUnit::Slot10 => 10,
            TextureUnit::Slot11 => 11,
            TextureUnit::Slot12 => 12,
            TextureUnit::Slot13 => 13,
            TextureUnit::Slot14 => 14,
            TextureUnit::Slot15 => 15,
        };
        uni_index.assign_to_current_program(gl_ctx, loc);
    }

}