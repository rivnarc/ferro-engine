use glow::HasContext;

pub struct Texture {
    pub id: glow::Texture,
}

impl Texture {
    pub fn from_image(gl: &glow::Context, path: &str) -> Self {
        Self::from_image_checked(gl, path)
            .expect(&format!("Не удалось загрузить текстуру: {}", path))
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }

    pub fn from_image_checked(gl: &glow::Context, path: &str) -> Option<Self> {
        if !std::path::Path::new(path).exists() {
            return None;
        }

        let img = image::open(path).ok()?.to_rgba8();
        let (width, height) = img.dimensions();

        unsafe {
            let id = gl.create_texture().expect("create_texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(id));

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(&img)),
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );

            gl.bind_texture(glow::TEXTURE_2D, None);
            Some(Self { id })
        }
    }
}
