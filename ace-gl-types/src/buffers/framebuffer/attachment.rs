use crate::gl;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FrameBufferAttachment {
    Color(u32),
    Depth,
    Stencil,
    DepthStencil
}

impl Into<u32> for FrameBufferAttachment {
    fn into(self) -> u32 {
        match self {
            FrameBufferAttachment::Color(c) => (gl::COLOR_ATTACHMENT0 + c),
            FrameBufferAttachment::Depth         => gl::DEPTH_ATTACHMENT,
            FrameBufferAttachment::Stencil       => gl::STENCIL_ATTACHMENT,
            FrameBufferAttachment::DepthStencil  => gl::DEPTH_STENCIL_ATTACHMENT,
        }
    }
}