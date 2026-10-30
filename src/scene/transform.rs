use glam::{Mat3, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform {
    pub fn to_matrix(&self) -> Mat3 {
        Mat3::from_translation(self.position)
            * Mat3::from_rotation_z(self.rotation)
            * Mat3::from_scale(self.scale)
    }
}
