pub mod gui;
pub mod hierarchy;
pub mod inspector;
pub mod viewport;

pub use gui::Gui;

use crate::engine::{Input, Time};
use crate::scene::{Scene, Transform};
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
            _ => false,
        }
    }

    pub fn update(&mut self, _input: &Input, _time: &Time, _scene: &mut Scene) {}
}
