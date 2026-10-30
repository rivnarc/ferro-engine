use crate::editor::{Editor, Gui};
use crate::engine::{GraphicsContext, Input, Time, WindowState};
use crate::graphics::Renderer2D;
use crate::scene::{Node, Scene, Transform};
use glam::Vec2;
use glow::HasContext;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
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
    fps: f32,
    last_print: Instant,
}

impl Default for App {
    fn default() -> Self {
        let mut scene = Scene::default();

        scene.add_node(Node {
            name: "Node".to_string(),
            transform: Transform {
                position: Vec2::new(0.0, 0.0),
                scale: Vec2::splat(256.0),
                ..Default::default()
            },
            ..Default::default()
        });

        Self {
            window: None,
            graphics: None,
            renderer: None,
            gui: None,
            time: Time::new(),
            input: Input::default(),
            scene,
            editor: Editor::default(),
            fps: 0.0,
            last_print: Instant::now(),
        }
    }
}

impl App {
    /// Размер окна в пикселях.
    fn viewport_size(&self) -> Vec2 {
        let s = self
            .window
            .as_ref()
            .map(|w| w.window.inner_size())
            .unwrap_or_default();
        Vec2::new(s.width as f32, s.height as f32)
    }

    /// Текущая view_proj матрица камеры.
    fn view_proj(&self) -> glam::Mat3 {
        let size = self.viewport_size();
        self.scene.camera.view_proj(size.x, size.y)
    }

    /// Обновление FPS (раз в кадр) + печать раз в секунду.
    fn update_fps(&mut self) {
        if self.time.delta > 0.0 {
            let instant = 1.0 / self.time.delta;
            self.fps = self.fps * 0.9 + instant * 0.1;
        }
        if self.last_print.elapsed().as_secs_f32() >= 1.0 {
            println!("FPS: {:.1}", self.fps);
            self.last_print = Instant::now();
        }
    }

    /// Обработка клика мышью: хит-тест + модификаторы (Ctrl/Shift).
    fn handle_click(&mut self) {
        if !self.input.take_click() {
            return;
        }
        let Some(win) = &self.window else {
            return;
        };
        let size = win.window.inner_size();
        let view_proj = self.view_proj();
        let mouse = self.input.mouse_position();

        let hits = crate::editor::viewport::hit_test_all(
            &self.scene,
            mouse,
            Vec2::new(size.width as f32, size.height as f32),
            view_proj,
        );

        let ctrl = self.input.ctrl;
        let shift = self.input.shift;

        if let Some(&top) = hits.first() {
            if ctrl {
                self.editor.toggle(top);
            } else if shift {
                self.editor.add(top);
            } else {
                self.editor.select_single(top);
            }
        } else if !ctrl && !shift {
            self.editor.selected.clear();
        }
    }

    fn render(&self, gl: &glow::Context, r: &Renderer2D) {
        let view_proj = self.view_proj();
        r.render_scene(gl, &self.scene, view_proj);
        for &idx in &self.editor.selected {
            if self.scene.nodes.get(idx).map(|n| n.alive).unwrap_or(false) {
                r.draw_selection_outline(gl, &self.scene, idx, view_proj);
            }
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
        // 1. Сначала egui (кроме RedrawRequested)
        if !matches!(event, WindowEvent::RedrawRequested) {
            if let (Some(gui), Some(win)) = (&mut self.gui, &self.window) {
                if gui.on_event(&win.window, &event) {
                    return;
                }
            }
        }

        // 2. Ввод движка
        if let Some(code) = self.input.process_event(&event) {
            if code == KeyCode::Escape {
                event_loop.exit();
                return;
            }
            self.editor.on_key(code, &mut self.scene);
        }

        // 3. События окна
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
                self.update_fps();

                // Камера
                let egui_wants_kb = self
                    .gui
                    .as_ref()
                    .map(|g| g.wants_keyboard())
                    .unwrap_or(false);
                crate::editor::viewport::update_camera(
                    &mut self.scene.camera,
                    &self.input,
                    self.time.delta,
                );
                let scroll = self.input.take_scroll();
                if scroll != 0.0 && !egui_wants_kb {
                    crate::editor::viewport::zoom_camera(&mut self.scene.camera, scroll);
                }

                // Редактор
                self.editor.update(&self.input, &self.time, &mut self.scene);
                self.handle_click();

                // Очистка + сцена + обводки
                if let (Some(g), Some(r)) = (&self.graphics, &self.renderer) {
                    unsafe {
                        g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
                        g.gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    self.render(&g.gl, r);
                }

                // GUI
                if let (Some(gui), Some(win)) = (&mut self.gui, &self.window) {
                    let scene = &mut self.scene;
                    let editor = &mut self.editor;
                    gui.run_ui(&win.window, |ctx| {
                        crate::editor::hierarchy::draw(ctx, scene, editor);
                        crate::editor::inspector::draw(ctx, scene, editor);
                    });
                    gui.paint(&win.window);
                }

                // Показ кадра
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
