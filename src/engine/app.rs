use crate::editor::Editor;
use crate::engine::{GraphicsContext, Input, Time, WindowState};
use crate::graphics::Renderer2D;
use crate::scene::{Node, Scene, Transform};
use glam::{Mat3, Vec2};
use glow::HasContext;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowId;

pub struct App {
    window: Option<WindowState>,
    graphics: Option<GraphicsContext>,
    renderer: Option<Renderer2D>,
    time: Time,
    input: Input,
    scene: Scene,
    editor: Editor,
}

impl Default for App {
    fn default() -> Self {
        let mut scene = Scene::default();

        let parent = scene.add_node(Node {
            _name: "Parent".to_string(),
            transform: Transform {
                position: Vec2::new(-0.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        let child = scene.add_node(Node {
            _name: "Child".to_string(),
            transform: Transform {
                position: Vec2::new(0.6, 0.0),
                scale: Vec2::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        });

        scene.attach(child, parent);

        Self {
            window: None,
            graphics: None,
            renderer: None,
            time: Time::new(),
            input: Input::default(),
            scene,
            editor: Editor::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_state = WindowState::new(event_loop, "Ferro Engine");
        let graphics = GraphicsContext::new(&window_state.window, window_state.gl_config.clone());
        let renderer = Renderer2D::new(&graphics.gl);

        let size = window_state.window.inner_size();
        unsafe {
            graphics
                .gl
                .viewport(0, 0, size.width as i32, size.height as i32);
        }

        self.window = Some(window_state);
        self.graphics = Some(graphics);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(code) = self.input.process_event(&event) {
            if code == KeyCode::Escape {
                event_loop.exit();
                return;
            }
            self.editor.on_key(code, &mut self.scene);
        }

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
                self.time.update();

                self.editor.update(&self.input, &self.time, &mut self.scene);

                if let (Some(g), Some(r)) = (&self.graphics, &self.renderer) {
                    unsafe {
                        g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
                        g.gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    r.render_scene(&g.gl, &self.scene, Mat3::IDENTITY);
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
