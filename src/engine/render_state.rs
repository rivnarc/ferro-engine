use crate::editor::Gui;
use crate::engine::{GraphicsContext, WindowState};
use crate::graphics::Renderer2D;
use glam::Vec2;
use glow::HasContext;
use winit::event_loop::ActiveEventLoop;

pub struct RenderState {
    pub gui: Option<Gui>,
    pub renderer: Option<Renderer2D>,
    pub graphics: Option<GraphicsContext>,
    pub window: Option<WindowState>,
}

impl RenderState {
    pub fn new(event_loop: &ActiveEventLoop, title: &str) -> Self {
        let window_state = WindowState::new(event_loop, title);
        let graphics = GraphicsContext::new(&window_state.window, window_state.gl_config.clone());
        let renderer = Renderer2D::new(&graphics.gl);
        let gui = Gui::new(event_loop, &window_state.window, graphics.gl.clone());

        let size = window_state.window.inner_size();
        unsafe {
            graphics
                .gl
                .viewport(0, 0, size.width as i32, size.height as i32);
        }

        Self {
            window: Some(window_state),
            graphics: Some(graphics),
            renderer: Some(renderer),
            gui: Some(gui),
        }
    }

    pub fn viewport_size(&self) -> Vec2 {
        let s = self
            .window
            .as_ref()
            .map(|w| w.window.inner_size())
            .unwrap_or_default();
        Vec2::new(s.width as f32, s.height as f32)
    }

    pub fn resize(&self, width: u32, height: u32) {
        if let Some(g) = &self.graphics {
            g.resize(width, height);
            unsafe {
                g.gl.viewport(0, 0, width as i32, height as i32);
            }
        }
    }

    pub fn wants_keyboard(&self) -> bool {
        self.gui
            .as_ref()
            .map(|g| g.wants_keyboard())
            .unwrap_or(false)
    }

    pub fn swap_buffers(&self) {
        if let Some(g) = &self.graphics {
            g.swap_buffers();
        }
    }

    pub fn request_redraw(&self) {
        if let Some(w) = &self.window {
            w.window.request_redraw();
        }
    }
}
