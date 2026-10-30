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
    selected: usize,
}

impl Default for App {
    fn default() -> Self {
        let mut scene = Scene::default();
        scene.nodes.push(Node {
            transform: Transform {
                position: Vec2::new(-0.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
        scene.nodes.push(Node {
            transform: Transform {
                position: Vec2::new(0.5, 0.0),
                scale: Vec2::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        });

        Self {
            window: None,
            graphics: None,
            renderer: None,
            time: Time::new(),
            input: Input::default(),
            scene,
            selected: 0,
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
            match code {
                KeyCode::Tab => {
                    self.selected = (self.selected + 1) % self.scene.nodes.len();
                }
                KeyCode::KeyR => {
                    self.scene.nodes[self.selected].transform = crate::scene::Transform::default();
                }
                KeyCode::Escape => event_loop.exit(),
                _ => (),
            }
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
                let mult = self.input.speed_multiplier();
                let move_speed = 1.5 * mult;
                let rot_speed = 3.0 * mult;
                let dt = self.time.delta;

                let node = &mut self.scene.nodes[self.selected];
                if self.input.is_pressed(KeyCode::ArrowLeft) {
                    node.transform.position.x -= move_speed * dt;
                }
                if self.input.is_pressed(KeyCode::ArrowRight) {
                    node.transform.position.x += move_speed * dt;
                }
                if self.input.is_pressed(KeyCode::ArrowUp) {
                    node.transform.position.y += move_speed * dt;
                }
                if self.input.is_pressed(KeyCode::ArrowDown) {
                    node.transform.position.y -= move_speed * dt;
                }
                if self.input.is_pressed(KeyCode::KeyQ) {
                    node.transform.rotation += rot_speed * dt;
                }
                if self.input.is_pressed(KeyCode::KeyE) {
                    node.transform.rotation -= rot_speed * dt;
                }
                let scale_speed = 1.0 * mult;
                if self.input.is_pressed(KeyCode::Equal)
                    || self.input.is_pressed(KeyCode::NumpadAdd)
                {
                    node.transform.scale += Vec2::splat(scale_speed * dt);
                }
                if self.input.is_pressed(KeyCode::Minus)
                    || self.input.is_pressed(KeyCode::NumpadSubtract)
                {
                    let new_scale = (node.transform.scale - Vec2::splat(scale_speed * dt))
                        .max(Vec2::splat(0.05));
                    node.transform.scale = new_scale;
                }

                if let (Some(g), Some(r)) = (&self.graphics, &self.renderer) {
                    unsafe {
                        g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
                        g.gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    let view_proj = Mat3::IDENTITY;
                    r.render_scene(&g.gl, &self.scene, view_proj);
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
