pub mod gui;
pub mod hierarchy;
pub mod inspector;
pub mod viewport;

pub use gui::Gui;

use crate::engine::{Input, Time};
use crate::scene::{Scene, Transform};
use winit::keyboard::KeyCode;

pub struct Editor {
    pub selected: Vec<usize>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            selected: Vec::new(),
        }
    }
}

impl Editor {
    pub fn select_single(&mut self, idx: usize) {
        self.selected.clear();
        self.selected.push(idx);
    }

    pub fn toggle(&mut self, idx: usize) {
        if let Some(pos) = self.selected.iter().position(|&x| x == idx) {
            self.selected.remove(pos);
        } else {
            self.selected.push(idx);
        }
    }

    pub fn add(&mut self, idx: usize) {
        if !self.selected.contains(&idx) {
            self.selected.push(idx);
        }
    }

    pub fn is_selected(&self, idx: usize) -> bool {
        self.selected.contains(&idx)
    }

    /// Первый выбранный узел (для инспектора). Возвращает None, если ничего не выбрано.
    pub fn primary(&self) -> Option<usize> {
        self.selected.first().copied()
    }

    pub fn on_key(&mut self, code: KeyCode, scene: &mut Scene) -> bool {
        match code {
            KeyCode::Tab => {
                if !scene.nodes.is_empty() {
                    let next = match self.primary() {
                        Some(i) => (i + 1) % scene.nodes.len(),
                        None => 0,
                    };
                    self.select_single(next);
                }
                true
            }
            KeyCode::KeyR => {
                for &i in &self.selected {
                    if let Some(node) = scene.nodes.get_mut(i) {
                        node.transform = Transform::default();
                    }
                }
                true
            }
            KeyCode::Delete => {
                for &i in &self.selected {
                    scene.remove_node(i);
                }
                self.selected.clear();
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self, _input: &Input, _time: &Time, _scene: &mut Scene) {}
}
