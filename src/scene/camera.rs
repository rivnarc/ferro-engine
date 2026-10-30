use glam::{Mat3, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Camera2D {
    pub position: Vec2,
    pub zoom: f32,
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Camera2D {
    pub fn view_proj(&self, width: f32, height: f32) -> Mat3 {
        let half_w = width * 0.5 / self.zoom;
        let half_h = height * 0.5 / self.zoom;
        let translate = Mat3::from_translation(-self.position);
        let scale = Mat3::from_scale(Vec2::new(1.0 / half_w, -1.0 / half_h));
        scale * translate
    }
}
