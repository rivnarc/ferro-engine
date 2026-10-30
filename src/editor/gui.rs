use egui_glow::EguiGlow;
use std::sync::Arc;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct Gui {
    pub egui_glow: EguiGlow,
}

impl Gui {
    pub fn new(event_loop: &ActiveEventLoop, _window: &Window, gl: Arc<glow::Context>) -> Self {
        let egui_glow = EguiGlow::new(event_loop, gl, None, None, true);
        Self { egui_glow }
    }

    pub fn on_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        self.egui_glow.on_window_event(window, event).consumed
    }

    pub fn run_ui(&mut self, window: &Window, run_ui: impl FnMut(&egui::Context)) {
        self.egui_glow.run(window, run_ui);
    }

    pub fn paint(&mut self, window: &Window) {
        self.egui_glow.paint(window);
    }
}
