pub mod buffer;
pub mod shader;
pub mod texture;
pub mod vertex_array;

use buffer::VertexBuffer;
use glam::{Mat3, Vec2};
use glow::HasContext;
use shader::ShaderProgram;
use std::collections::HashMap;
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
uniform vec4 uColor;
uniform bool uUseTexture;
void main() {
    if (uUseTexture) {
        FragColor = texture(uTexture, UV);
    } else {
        FragColor = uColor;
    }
}
"#;

pub struct Renderer2D {
    program: ShaderProgram,
    vao: VertexArray,
    _vbo: VertexBuffer,
    textures: HashMap<String, Texture>,
    u_model_loc: glow::UniformLocation,
    u_viewproj_loc: glow::UniformLocation,
    u_color_loc: glow::UniformLocation,
    u_use_texture_loc: glow::UniformLocation,
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
        let mut textures = HashMap::new();
        if let Ok(entries) = std::fs::read_dir("assets/textures") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".png") {
                        let path = format!("assets/textures/{}", name);
                        if let Some(tex) = Texture::from_image_checked(gl, &path) {
                            textures.insert(name.to_string(), tex);
                        }
                    }
                }
            }
        }
        let default = Texture::from_image(gl, "assets/textures/prototype_texture.png");
        textures.insert("__default__".to_string(), default);

        let (u_model_loc, u_viewproj_loc) = unsafe {
            (
                gl.get_uniform_location(program.program, "uModel")
                    .expect("uModel"),
                gl.get_uniform_location(program.program, "uViewProj")
                    .expect("uViewProj"),
            )
        };

        let u_color_loc = unsafe {
            gl.get_uniform_location(program.program, "uColor")
                .expect("uColor")
        };
        let u_use_texture_loc = unsafe {
            gl.get_uniform_location(program.program, "uUseTexture")
                .expect("uUseTexture")
        };

        Self {
            program,
            vao,
            _vbo: vbo,
            textures,
            u_model_loc,
            u_viewproj_loc,
            u_color_loc,
            u_use_texture_loc,
        }
    }

    pub fn render(
        &self,
        gl: &glow::Context,
        model: Mat3,
        view_proj: Mat3,
        color: [f32; 4],
        use_texture: bool,
        texture: &Texture,
    ) {
        self.program.bind(gl);
        texture.bind(gl);
        self.vao.bind(gl);
        unsafe {
            gl.uniform_matrix_3_f32_slice(Some(&self.u_model_loc), false, &model.to_cols_array());
            gl.uniform_matrix_3_f32_slice(
                Some(&self.u_viewproj_loc),
                false,
                &view_proj.to_cols_array(),
            );
            gl.uniform_4_f32(
                Some(&self.u_color_loc),
                color[0],
                color[1],
                color[2],
                color[3],
            );
            gl.uniform_1_i32(
                Some(&self.u_use_texture_loc),
                if use_texture { 1 } else { 0 },
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
        let default = self.textures.get("__default__").expect("default texture");

        for i in 0..scene.nodes.len() {
            if !scene.nodes[i].alive {
                continue;
            }
            let model = scene.world_matrix(i);
            let tex_name = scene.nodes[i].texture.as_deref().unwrap_or("__default__");
            let tex = self.textures.get(tex_name).unwrap_or(default);
            self.render(gl, model, view_proj, [1.0, 1.0, 1.0, 1.0], true, tex);
        }
    }

    pub fn draw_selection_outline(
        &self,
        gl: &glow::Context,
        scene: &crate::scene::Scene,
        index: usize,
        view_proj: glam::Mat3,
    ) {
        let world = scene.world_matrix(index);

        let scale = scene.nodes[index].transform.scale;
        let thickness = 2.0 / scale.x.max(scale.y);
        let half = 0.5;

        let color = [1.0, 0.6, 0.0, 1.0];

        let rects = [
            (
                glam::Vec2::new(0.0, -half - thickness * 0.5),
                glam::Vec2::new(1.0 + thickness * 2.0, thickness),
            ),
            (
                glam::Vec2::new(0.0, half + thickness * 0.5),
                glam::Vec2::new(1.0 + thickness * 2.0, thickness),
            ),
            (
                glam::Vec2::new(-half - thickness * 0.5, 0.0),
                glam::Vec2::new(thickness, 1.0),
            ),
            (
                glam::Vec2::new(half + thickness * 0.5, 0.0),
                glam::Vec2::new(thickness, 1.0),
            ),
        ];

        for (local_pos, local_size) in rects {
            let local =
                glam::Mat3::from_translation(local_pos) * glam::Mat3::from_scale(local_size);
            let model = world * local;
            let default = self.textures.get("__default__").expect("default");
            self.render(gl, model, view_proj, color, false, default);
        }
    }
    pub fn draw_world_rect(
        &self,
        gl: &glow::Context,
        pos: glam::Vec2,
        size: glam::Vec2,
        view_proj: glam::Mat3,
        color: [f32; 4],
    ) {
        let model = glam::Mat3::from_translation(pos) * glam::Mat3::from_scale(size);
        let default = self.textures.get("__default__").expect("default texture");
        self.render(gl, model, view_proj, color, false, default);
    }
    pub fn draw_collider(
        &self,
        gl: &glow::Context,
        scene: &crate::scene::Scene,
        index: usize,
        view_proj: glam::Mat3,
    ) {
        let Some(aabb) = crate::physics::collision::node_aabb(scene, index) else {
            return;
        };

        let color = [0.0, 1.0, 0.0, 1.0];
        let thickness = 2.0;

        let center = aabb.center;
        let half = aabb.half_size;

        let rects = [
            (
                center + Vec2::new(0.0, -half.y),
                Vec2::new(half.x * 2.0 + thickness, thickness),
            ),
            (
                center + Vec2::new(0.0, half.y),
                Vec2::new(half.x * 2.0 + thickness, thickness),
            ),
            (
                center + Vec2::new(-half.x, 0.0),
                Vec2::new(thickness, half.y * 2.0),
            ),
            (
                center + Vec2::new(half.x, 0.0),
                Vec2::new(thickness, half.y * 2.0),
            ),
        ];

        for (pos, size) in rects {
            self.draw_world_rect(gl, pos, size, view_proj, color);
        }
    }
}
