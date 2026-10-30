use crate::engine::{Input, Time};
use crate::scene::{Scene, Transform};
use glam::Vec2;
use winit::keyboard::KeyCode;

pub struct Editor {
    pub selected: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self { selected: 0 }
    }
}

impl Editor {
    pub fn on_key(&mut self, code: KeyCode, scene: &mut Scene) -> bool {
        match code {
            KeyCode::Tab => {
                if !scene.nodes.is_empty() {
                    self.selected = (self.selected + 1) % scene.nodes.len();
                }
                true
            }
            KeyCode::KeyR => {
                if let Some(node) = scene.nodes.get_mut(self.selected) {
                    node.transform = Transform::default();
                }
                true
            }
            KeyCode::KeyP => {
                if self.selected > 0 {
                    scene.attach(self.selected, self.selected - 1);
                }
                true
            }
            KeyCode::KeyO => {
                scene.detach(self.selected);
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self, input: &Input, time: &Time, scene: &mut Scene) {
        let Some(node) = scene.nodes.get_mut(self.selected) else {
            return;
        };

        let mult = input.speed_multiplier();
        let dt = time.delta;
        let move_speed = 1.5 * mult;
        let rot_speed = 3.0 * mult;
        let scale_speed = 1.0 * mult;

        if input.is_pressed(KeyCode::ArrowLeft) {
            node.transform.position.x -= move_speed * dt;
        }
        if input.is_pressed(KeyCode::ArrowRight) {
            node.transform.position.x += move_speed * dt;
        }
        if input.is_pressed(KeyCode::ArrowUp) {
            node.transform.position.y += move_speed * dt;
        }
        if input.is_pressed(KeyCode::ArrowDown) {
            node.transform.position.y -= move_speed * dt;
        }
        if input.is_pressed(KeyCode::KeyQ) {
            node.transform.rotation += rot_speed * dt;
        }
        if input.is_pressed(KeyCode::KeyE) {
            node.transform.rotation -= rot_speed * dt;
        }

        if input.is_pressed(KeyCode::Equal) || input.is_pressed(KeyCode::NumpadAdd) {
            node.transform.scale += Vec2::splat(scale_speed * dt);
        }
        if input.is_pressed(KeyCode::Minus) || input.is_pressed(KeyCode::NumpadSubtract) {
            let new_scale =
                (node.transform.scale - Vec2::splat(scale_speed * dt)).max(Vec2::splat(0.05));
            node.transform.scale = new_scale;
        }
    }
}
