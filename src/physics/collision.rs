use crate::physics::Aabb;
use crate::scene::Scene;

pub fn node_aabb(scene: &Scene, idx: usize) -> Option<Aabb> {
    let node = scene.nodes.get(idx)?;
    if !node.alive {
        return None;
    }
    let size = node.collider?;
    let center = scene.world_matrix(idx).transform_point2(glam::Vec2::ZERO);
    Some(Aabb::new(center, size))
}

pub fn collides(scene: &Scene, a: usize, b: usize) -> bool {
    if a == b {
        return false;
    }
    let (Some(aa), Some(bb)) = (node_aabb(scene, a), node_aabb(scene, b)) else {
        return false;
    };
    aa.intersects(&bb)
}

pub fn collisions_for(scene: &Scene, idx: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..scene.nodes.len() {
        if i != idx && collides(scene, idx, i) {
            result.push(i);
        }
    }
    result
}
