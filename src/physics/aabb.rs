use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub center: Vec2,
    pub half_size: Vec2,
}

impl Aabb {
    pub fn new(center: Vec2, size: Vec2) -> Self {
        Self {
            center,
            half_size: size * 0.5,
        }
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        (self.center.x - self.half_size.x) < (other.center.x + other.half_size.x)
            && (self.center.x + self.half_size.x) > (other.center.x - other.half_size.x)
            && (self.center.y - self.half_size.y) < (other.center.y + other.half_size.y)
            && (self.center.y + self.half_size.y) > (other.center.y - other.half_size.y)
    }
}
