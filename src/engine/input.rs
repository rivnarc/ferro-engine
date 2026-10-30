use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Default)]
pub struct Input {
    pressed: HashSet<KeyCode>,
    pub shift: bool,
}

impl Input {
    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.pressed.contains(&code)
    }

    pub fn process_event(&mut self, event: &WindowEvent) -> Option<KeyCode> {
        if let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    repeat,
                    ..
                },
            ..
        } = event
        {
            match state {
                ElementState::Pressed => {
                    if !repeat {
                        self.pressed.insert(*code);
                    }
                    if *code == KeyCode::ShiftLeft || *code == KeyCode::ShiftRight {
                        self.shift = true;
                    }
                    if !repeat {
                        return Some(*code);
                    }
                }
                ElementState::Released => {
                    self.pressed.remove(code);
                    if *code == KeyCode::ShiftLeft || *code == KeyCode::ShiftRight {
                        self.shift = false;
                    }
                }
            }
        }
        None
    }

    pub fn speed_multiplier(&self) -> f32 {
        if self.shift { 2.0 } else { 1.0 }
    }
}
