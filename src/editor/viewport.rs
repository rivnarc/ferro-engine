use crate::engine::Input;
use crate::scene::Camera2D;
use glam::Vec2;
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
