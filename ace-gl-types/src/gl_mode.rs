use crate::gl;

pub enum GLMode {
    Points                  = gl::POINTS as isize,
    LineStrip               = gl::LINE_STRIP as isize,
    LineLoop                = gl::LINE_LOOP as isize,
    Lines                   = gl::LINES as isize,
    LineStripAdjacency      = gl::LINE_STRIP_ADJACENCY as isize,
    LinesAdjacency          = gl::LINES_ADJACENCY as isize,
    TriangleStrip           = gl::TRIANGLE_STRIP as isize,
    TriangleFan             = gl::TRIANGLE_FAN as isize,
    Triangles               = gl::TRIANGLES as isize,
    TriangleStripAdjacency  = gl::TRIANGLE_STRIP_ADJACENCY as isize,
    TrianglesAdjacency      = gl::TRIANGLES_ADJACENCY as isize,
    Patches                 = gl::PATCHES as isize,
}