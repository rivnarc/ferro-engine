use glow::HasContext;

pub struct VertexBuffer {
    #[allow(dead_code)]
    pub id: glow::Buffer,
}

impl VertexBuffer {
    pub fn new_f32(gl: &glow::Context, data: &[f32]) -> Self {
        unsafe {
            let id = gl.create_buffer().expect("create_buffer");
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(id));
            let bytes = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::STATIC_DRAW);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            Self { id }
        }
    }
    #[allow(dead_code)]
    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
        }
    }
}
