use crate::math;
use glam::{Mat3, Vec2};

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
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
        let half_size = Vec2::new(width * 0.5 / self.zoom, height * 0.5 / self.zoom);
        math::ortho_2d(self.position, half_size)
    }
}
