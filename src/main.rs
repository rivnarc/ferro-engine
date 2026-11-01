mod engine;
mod graphics;
mod math;
mod physics;
mod scene;

#[cfg(feature = "editor")]
mod editor;

use engine::app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_game_mode = args.len() > 1;

    if is_game_mode {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                std::env::set_current_dir(dir).ok();
            }
        }
    }

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = if is_game_mode {
        App::new_game(args[1].clone())
    } else {
        #[cfg(feature = "editor")]
        {
            App::new_editor()
        }
        #[cfg(not(feature = "editor"))]
        {
            App::new_game("assets/scenes/level1.ron".to_string())
        }
    };

    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Ошибка event loop: {}", e);
    }
}
