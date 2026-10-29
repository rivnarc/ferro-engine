pub mod buffer;
pub mod shader;
pub mod texture;
pub mod vertex_array;

use buffer::VertexBuffer;
use glow::HasContext;
use shader::ShaderProgram;
use vertex_array::VertexArray;

const VERTEX_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 aPos;
    void main() {
        gl_Position = vec4(aPos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SRC: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(0.9, 0.3, 0.3, 1.0);
    }
"#;

pub struct Renderer2D {
    program: ShaderProgram,
    vao: VertexArray,
    _vbo: VertexBuffer,
}

impl Renderer2D {
    pub fn new(gl: &glow::Context) -> Self {
        let program = ShaderProgram::new(gl, VERTEX_SRC, FRAGMENT_SRC);

        let vertices: [f32; 6] = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
        let vbo = VertexBuffer::new_f32(gl, &vertices);

        let vao = VertexArray::new(gl);
        vao.set_attrib_f32(gl, &vbo, 0, 2, 8, 0);

        Self {
            program,
            vao,
            _vbo: vbo,
        }
    }

    pub fn render_triangle(&self, gl: &glow::Context) {
        self.program.bind(gl);
        self.vao.bind(gl);
        unsafe {
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
        VertexArray::unbind(gl);
    }
}
