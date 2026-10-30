pub mod buffer;
pub mod shader;
pub mod texture;
pub mod vertex_array;

use buffer::VertexBuffer;
use glam::Mat3;
use glow::HasContext;
use shader::ShaderProgram;
use texture::Texture;
use vertex_array::VertexArray;

const VERTEX_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec2 aUV;
    out vec2 UV;
    uniform mat3 uModel;
    uniform mat3 uViewProj;
    void main() {
        vec3 world = uModel * vec3(aPos, 1.0);
        vec3 clip = uViewProj * world;
        gl_Position = vec4(clip.xy, 0.0, 1.0);
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
    u_model_loc: glow::UniformLocation,
    u_viewproj_loc: glow::UniformLocation,
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

        let (u_model_loc, u_viewproj_loc) = unsafe {
            (
                gl.get_uniform_location(program.program, "uModel")
                    .expect("uModel"),
                gl.get_uniform_location(program.program, "uViewProj")
                    .expect("uViewProj"),
            )
        };

        Self {
            program,
            vao,
            _vbo: vbo,
            texture,
            u_model_loc,
            u_viewproj_loc,
        }
    }

    pub fn render(&self, gl: &glow::Context, model: Mat3, view_proj: Mat3) {
        self.program.bind(gl);
        self.texture.bind(gl);
        self.vao.bind(gl);

        unsafe {
            gl.uniform_matrix_3_f32_slice(Some(&self.u_model_loc), false, &model.to_cols_array());
            gl.uniform_matrix_3_f32_slice(
                Some(&self.u_viewproj_loc),
                false,
                &view_proj.to_cols_array(),
            );

            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
        VertexArray::unbind(gl);
    }

    pub fn render_scene(
        &self,
        gl: &glow::Context,
        scene: &crate::scene::Scene,
        view_proj: glam::Mat3,
    ) {
        for i in 0..scene.nodes.len() {
            let model = scene.world_matrix(i);
            self.render(gl, model, view_proj);
        }
    }
}
