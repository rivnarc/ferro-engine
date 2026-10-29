use crate::graphics::buffer::VertexBuffer;
use glow::HasContext;

pub struct VertexArray {
    pub id: glow::VertexArray,
}

impl VertexArray {
    pub fn new(gl: &glow::Context) -> Self {
        unsafe {
            let id = gl.create_vertex_array().expect("create_vertex_array");
            Self { id }
        }
    }

    pub fn set_attrib_f32(
        &self,
        gl: &glow::Context,
        vbo: &VertexBuffer,
        location: u32,
        components: i32,
        stride: i32,
        offset: i32,
    ) {
        unsafe {
            gl.bind_vertex_array(Some(self.id));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo.id));
            gl.vertex_attrib_pointer_f32(location, components, glow::FLOAT, false, stride, offset);
            gl.enable_vertex_attrib_array(location);
            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.id));
        }
    }

    pub fn unbind(gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(None);
        }
    }
}
