use crate::gl;

pub enum DrawMode {
    StreamDraw = gl::STREAM_DRAW as isize,
    StreamRead = gl::STREAM_READ as isize,
    StreamCopy = gl::STREAM_COPY as isize,
    StaticDraw = gl::STATIC_DRAW as isize,
    StaticRead = gl::STATIC_READ as isize,
    StaticCopy = gl::STATIC_COPY as isize,
    DynamicDraw = gl::DYNAMIC_DRAW as isize,
    DynamicRead = gl::DYNAMIC_READ as isize,
    DynamicCopy = gl::DYNAMIC_COPY as isize,
}