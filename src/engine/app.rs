use crate::editor::{Editor, Gui};
use crate::engine::{GraphicsContext, Input, Time, WindowState};
use crate::graphics::Renderer2D;
use crate::scene::{Node, Scene, Transform};
use glam::{Mat3, Vec2};
use glow::HasContext;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct App {
    gui: Option<Gui>,
    renderer: Option<Renderer2D>,
    graphics: Option<GraphicsContext>,
    window: Option<WindowState>,
    time: Time,
    input: Input,
    scene: Scene,
    editor: Editor,
}

impl Default for App {
    fn default() -> Self {
        let mut scene = Scene::default();
        let node = scene.add_node(Node {
            name: "Node".to_string(),
            transform: Transform {
                position: Vec2::new(-0.5, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
        let node1 = scene.add_node(Node {
            name: "Node1".to_string(),
            transform: Transform {
                position: Vec2::new(0.6, 0.0),
                scale: Vec2::splat(0.5),
                ..Default::default()
            },
            ..Default::default()
        });
        scene.attach(node1, node);

        Self {
            window: None,
            graphics: None,
            renderer: None,
            gui: None,
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
        let gui = Gui::new(event_loop, &window_state.window, graphics.gl.clone());

        let size = window_state.window.inner_size();
        unsafe {
            graphics
                .gl
                .viewport(0, 0, size.width as i32, size.height as i32);
        }

        self.window = Some(window_state);
        self.graphics = Some(graphics);
        self.renderer = Some(renderer);
        self.gui = Some(gui);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if !matches!(event, WindowEvent::RedrawRequested) {
            if let (Some(gui), Some(win)) = (&mut self.gui, &self.window) {
                if gui.on_event(&win.window, &event) {
                    return;
                }
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
                self.editor.update(&self.input, &self.time, &mut self.scene);

                if let Some(g) = &self.graphics {
                    unsafe {
                        g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
                        g.gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                }

                if let (Some(g), Some(r)) = (&self.graphics, &self.renderer) {
                    r.render_scene(&g.gl, &self.scene, Mat3::IDENTITY);
                }

                if let (Some(gui), Some(win)) = (&mut self.gui, &self.window) {
                    let scene = &mut self.scene;
                    let selected = &mut self.editor.selected;

                    gui.run_ui(&win.window, |ctx| {
                        if let Some(new_selected) =
                            crate::editor::hierarchy::draw(ctx, scene, *selected)
                        {
                            *selected = new_selected;
                        }
                        crate::editor::inspector::draw(ctx, scene, *selected);
                    });

                    gui.paint(&win.window);
                }

                if let Some(g) = &self.graphics {
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
