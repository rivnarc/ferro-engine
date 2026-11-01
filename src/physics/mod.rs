pub mod aabb;
pub mod collision;

pub use aabb::Aabb;
pub use collision::collisions_for;

use crate::scene::Scene;
use glam::Vec2;

pub const GRAVITY: f32 = 980.0;

pub fn integrate_and_resolve(scene: &mut Scene, dt: f32) {
    let max_step = 1.0 / 120.0;
    let steps = (dt / max_step).ceil() as usize;
    let step_dt = dt / steps.max(1) as f32;

    for _ in 0..steps.max(1) {
        integrate(scene, step_dt);
        resolve_collisions(scene);
    }
}

pub fn integrate(scene: &mut Scene, dt: f32) {
    for node in &mut scene.nodes {
        if !node.alive {
            continue;
        }
        let Some(body) = &mut node.physics else {
            continue;
        };

        node.previous_position = node.transform.position;

        if body.gravity && !body.grounded {
            body.velocity.y += GRAVITY * dt;
        }

        node.transform.position += body.velocity * dt;
    }
}

pub fn resolve_collisions(scene: &mut Scene) {
    let len = scene.nodes.len();

    for node in &mut scene.nodes {
        if let Some(body) = &mut node.physics {
            body.grounded = false;
        }
    }

    let mut to_resolve: Vec<usize> = Vec::new();
    for i in 0..len {
        if scene.nodes[i].physics.is_none() || !scene.nodes[i].alive {
            continue;
        }
        if !collisions_for(scene, i).is_empty() {
            to_resolve.push(i);
        }
    }
    for i in to_resolve {
        let prev = scene.nodes[i].previous_position;
        if let Some(body) = &mut scene.nodes[i].physics {
            if body.velocity.y >= 0.0 {
                body.grounded = true;
            }
            body.velocity = Vec2::ZERO;
        }
        scene.nodes[i].transform.position = prev;
    }
}
