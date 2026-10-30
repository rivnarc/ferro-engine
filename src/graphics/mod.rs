pub mod buffer;
pub mod shader;
pub mod texture;
pub mod vertex_array;

use buffer::VertexBuffer;
use glow::HasContext;
use shader::ShaderProgram;
use texture::Texture;
use vertex_array::VertexArray;

const VERTEX_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec2 aUV;
    out vec2 UV;
    void main() {
        gl_Position = vec4(aPos, 0.0, 1.0);
        UV = aUV;
    }
"#;

const FRAGMENT_SRC: &str = r#"
    #version 330 core
    in vec2 UV;
    out vec4 FragColor;
    uniform sampler2D uTexture;
    void main() {
        FragColor = texture(uTexture, UV);
    }
"#;

pub struct Renderer2D {
    program: ShaderProgram,
    vao: VertexArray,
    _vbo: VertexBuffer,
    texture: Texture,
}

impl Renderer2D {
    pub fn new(gl: &glow::Context) -> Self {
        let program = ShaderProgram::new(gl, VERTEX_SRC, FRAGMENT_SRC);

        let vertices: [f32; 24] = [
            -0.5, 0.5, 0.0, 1.0, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, 1.0, 0.0, -0.5, 0.5, 0.0, 1.0,
            0.5, -0.5, 1.0, 0.0, 0.5, 0.5, 1.0, 1.0,
        ];

        let vbo = VertexBuffer::new_f32(gl, &vertices);
        let vao = VertexArray::new(gl);
        vao.set_attrib_f32(gl, &vbo, 0, 2, 16, 0);
        vao.set_attrib_f32(gl, &vbo, 1, 2, 16, 8);

        let texture = Texture::from_image(gl, "assets/textures/prototype_texture.png");

        Self {
            program,
            vao,
            _vbo: vbo,
            texture,
        }
    }

    pub fn render(&self, gl: &glow::Context) {
        self.program.bind(gl);
        self.texture.bind(gl);
        self.vao.bind(gl);
        unsafe {
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
        VertexArray::unbind(gl);
    }
}
