use crate::engine::{Input, Time};
use crate::scene::Scene;

pub mod gui;
pub mod hierarchy;
pub mod inspector;

pub use gui::Gui;

pub struct Editor {
    pub selected: usize,
}

impl Default for Editor {
    fn default() -> Self {
        Self { selected: 0 }
    }
}

impl Editor {
    pub fn update(&mut self, _input: &Input, _time: &Time, _scene: &mut Scene) {}
}
