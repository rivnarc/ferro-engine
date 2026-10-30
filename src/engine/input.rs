use glam::Vec2;
use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Default)]
pub struct Input {
    pressed: HashSet<KeyCode>,
    pub shift: bool,
    pub ctrl: bool,
    scroll_delta: f32,
    mouse_pos: Vec2,
    mouse_clicked: bool,
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

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_pos
    }

    pub fn take_click(&mut self) -> bool {
        let c = self.mouse_clicked;
        self.mouse_clicked = false;
        c
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
        if let WindowEvent::CursorMoved { position, .. } = event {
            self.mouse_pos = Vec2::new(position.x as f32, position.y as f32);
        }
        if let WindowEvent::MouseInput {
            state,
            button: winit::event::MouseButton::Left,
            ..
        } = event
        {
            if *state == winit::event::ElementState::Pressed {
                self.mouse_clicked = true;
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
                    if *code == KeyCode::ControlLeft || *code == KeyCode::ControlRight {
                        self.ctrl = true;
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
                    if *code == KeyCode::ControlLeft || *code == KeyCode::ControlRight {
                        self.ctrl = false;
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
