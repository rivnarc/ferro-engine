use glow::HasContext;

pub struct ShaderProgram {
    pub program: glow::Program,
}

impl ShaderProgram {
    pub fn new(gl: &glow::Context, vertex_src: &str, fragment_src: &str) -> Self {
        unsafe {
            let program = gl.create_program().expect("create_program");

            let v_shader = Self::compile(gl, glow::VERTEX_SHADER, vertex_src);
            let f_shader = Self::compile(gl, glow::FRAGMENT_SHADER, fragment_src);

            gl.attach_shader(program, v_shader);
            gl.attach_shader(program, f_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!("Shader link error: {}", gl.get_program_info_log(program));
            }

            gl.detach_shader(program, v_shader);
            gl.detach_shader(program, f_shader);
            gl.delete_shader(v_shader);
            gl.delete_shader(f_shader);

            Self { program }
        }
    }

    fn compile(gl: &glow::Context, kind: u32, src: &str) -> glow::Shader {
        unsafe {
            let shader = gl.create_shader(kind).expect("create_shader");
            gl.shader_source(shader, src);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("Shader compile error: {}", gl.get_shader_info_log(shader));
            }
            shader
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {}
}
