use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Default)]
pub struct Input {
    pressed: HashSet<KeyCode>,
    pub shift: bool,
    scroll_delta: f32,
}

impl Input {
    pub fn is_pressed(&self, code: KeyCode) -> bool {
        self.pressed.contains(&code)
    }

    pub fn take_scroll(&mut self) -> f32 {
        let s = self.scroll_delta;
        self.scroll_delta = 0.0;
        s
    }

    pub fn process_event(&mut self, event: &WindowEvent) -> Option<KeyCode> {
        if let WindowEvent::MouseWheel { delta, .. } = event {
            match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => self.scroll_delta += y,
                winit::event::MouseScrollDelta::PixelDelta(pos) => {
                    self.scroll_delta += pos.y as f32 / 50.0
                }
            }
        }
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
