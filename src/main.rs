mod editor;
mod engine;
mod graphics;
mod math;
mod scene;

use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = engine::app::App::default();

    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Ошибка event loop: {}", e);
    }
}
