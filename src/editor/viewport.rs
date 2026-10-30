use crate::engine::Input;
use crate::scene::{Camera2D, Scene};
use glam::{Mat3, Vec2};
use winit::keyboard::KeyCode;

pub fn update_camera(camera: &mut Camera2D, input: &Input, dt: f32) {
    let mult = input.speed_multiplier();
    let speed = 500.0 * mult;

    let mut dir = Vec2::ZERO;
    if input.is_pressed(KeyCode::KeyW) {
        dir.y -= 1.0;
    }
    if input.is_pressed(KeyCode::KeyS) {
        dir.y += 1.0;
    }
    if input.is_pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if input.is_pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if dir.length_squared() > 0.0 {
        camera.position += dir.normalize() * speed * dt / camera.zoom;
    }
}

pub fn zoom_camera(camera: &mut Camera2D, delta: f32) {
    let factor = if delta > 0.0 { 1.1 } else { 1.0 / 1.1 };
    camera.zoom = (camera.zoom * factor).clamp(0.1, 10.0);
}

pub fn hit_test_all(
    scene: &Scene,
    screen_pos: Vec2,
    viewport_size: Vec2,
    view_proj: Mat3,
) -> Vec<usize> {
    let ndc = Vec2::new(
        (screen_pos.x / viewport_size.x) * 2.0 - 1.0,
        1.0 - (screen_pos.y / viewport_size.y) * 2.0,
    );
    let inv = view_proj.inverse();
    let world = inv.transform_point2(ndc);

    let mut hits = Vec::new();
    for i in (0..scene.nodes.len()).rev() {
        let node = &scene.nodes[i];
        if !node.alive {
            continue;
        }
        let model = scene.world_matrix(i);
        let inv_model = model.inverse();
        let local = inv_model.transform_point2(world);
        if local.x.abs() <= 0.5 && local.y.abs() <= 0.5 {
            hits.push(i);
        }
    }
    hits
}
