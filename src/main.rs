mod editor;
mod engine;
mod graphics;
mod math;
mod scene;

use engine::{GraphicsContext, WindowState};
use glow::HasContext;
use graphics::Renderer2D;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

#[derive(Default)]
struct App {
    window: Option<WindowState>,
    graphics: Option<GraphicsContext>,
    renderer: Option<Renderer2D>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_state = WindowState::new(event_loop, "Ferro Engine");
        let graphics = GraphicsContext::new(&window_state.window, window_state.gl_config.clone());
        let renderer = Renderer2D::new(&graphics.gl);

        if let Some(g) = &self.graphics {
            unsafe {
                let size = window_state.window.inner_size();
                g.gl.viewport(0, 0, size.width as i32, size.height as i32);
            }
        }

        self.window = Some(window_state);
        self.graphics = Some(graphics);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(g) = &self.graphics {
                    g.resize(size.width, size.height);
                    unsafe {
                        g.gl.viewport(0, 0, size.width as i32, size.height as i32);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(g), Some(r)) = (&self.graphics, &self.renderer) {
                    unsafe {
                        g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
                        g.gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    r.render(&g.gl);
                    g.swap_buffers();
                }
                if let Some(w) = &self.window {
                    w.window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Ошибка event loop: {}", e);
    }
}
