use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Default)]
pub struct Input {
    _pressed: HashSet<KeyCode>,
    pub _shift: bool,
}

impl Input {
    pub fn _is_pressed(&self, code: KeyCode) -> bool {
        self._pressed.contains(&code)
    }

    pub fn _process_event(&mut self, event: &WindowEvent) -> Option<KeyCode> {
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
                        self._pressed.insert(*code);
                    }
                    if *code == KeyCode::ShiftLeft || *code == KeyCode::ShiftRight {
                        self._shift = true;
                    }
                    if !repeat {
                        return Some(*code);
                    }
                }
                ElementState::Released => {
                    self._pressed.remove(code);
                    if *code == KeyCode::ShiftLeft || *code == KeyCode::ShiftRight {
                        self._shift = false;
                    }
                }
            }
        }
        None
    }

    pub fn _speed_multiplier(&self) -> f32 {
        if self._shift { 2.0 } else { 1.0 }
    }
}
