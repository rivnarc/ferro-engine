use crate::editor::{Editor, Gui};
use crate::engine::{GraphicsContext, Input, Time, WindowState};
use crate::graphics::Renderer2D;
use crate::scene::{Node, Scene, Transform};
use glam::Vec2;
use glow::HasContext;
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
        }
    }
}

impl App {
    fn viewport_size(&self) -> Vec2 {
        let s = self
            .window
            .as_ref()
            .map(|w| w.window.inner_size())
            .unwrap_or_default();
        Vec2::new(s.width as f32, s.height as f32)
    }

    fn view_proj(&self) -> glam::Mat3 {
        let size = self.viewport_size();
        self.scene.camera.view_proj(size.x, size.y)
    }

    fn handle_click(&mut self) {
        if !self.input.take_click() {
            return;
        }
        let Some(win) = &self.window else {
            return;
        };
        let size = win.window.inner_size();
        let vp_size = Vec2::new(size.width as f32, size.height as f32);
        let view_proj = self.view_proj();
        let mouse_screen = self.input.mouse_position();
        let mouse_world =
            crate::editor::viewport::screen_to_world(mouse_screen, vp_size, view_proj);

        if let Some(axis) =
            crate::editor::gizmo::hit_test_gizmo(&self.scene, &self.editor.selected, mouse_world)
        {
            self.editor.drag = Some(crate::editor::gizmo::GizmoDrag::new(
                axis,
                mouse_world,
                &self.scene,
                &self.editor.selected,
            ));
            return;
        }

        let hits =
            crate::editor::viewport::hit_test_all(&self.scene, mouse_screen, vp_size, view_proj);
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

    fn update_drag(&mut self) {
        let Some(drag) = &self.editor.drag else {
            return;
        };
        if !self.input.mouse_down {
            self.editor.drag = None;
            return;
        }
        let Some(win) = &self.window else {
            return;
        };
        let size = win.window.inner_size();
        let vp_size = Vec2::new(size.width as f32, size.height as f32);
        let view_proj = self.view_proj();
        let mouse_screen = self.input.mouse_position();
        let mouse_world =
            crate::editor::viewport::screen_to_world(mouse_screen, vp_size, view_proj);
        drag.update(&mut self.scene, mouse_world);
    }

    fn render(&self, gl: &glow::Context, r: &Renderer2D) {
        let view_proj = self.view_proj();

        r.render_scene(gl, &self.scene, view_proj);

        for &idx in &self.editor.selected {
            if self.scene.nodes.get(idx).map(|n| n.alive).unwrap_or(false) {
                r.draw_selection_outline(gl, &self.scene, idx, view_proj);
            }
        }

        if !self.editor.selected.is_empty() {
            crate::editor::gizmo::draw_gizmo(
                &self.scene,
                &self.editor.selected,
                |pos, size, color| {
                    r.draw_world_rect(gl, pos, size, view_proj, color);
                },
            );
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

                self.editor.update(&self.input, &self.time, &mut self.scene);
                self.handle_click();
                self.update_drag();

                if let (Some(g), Some(r)) = (&self.graphics, &self.renderer) {
                    unsafe {
                        g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
                        g.gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    self.render(&g.gl, r);
                }

                if let (Some(gui), Some(win)) = (&mut self.gui, &self.window) {
                    let scene = &mut self.scene;
                    let editor = &mut self.editor;
                    gui.run_ui(&win.window, |ctx| {
                        crate::editor::hierarchy::draw(ctx, scene, editor);
                        crate::editor::inspector::draw(ctx, scene, editor);
                        crate::editor::savenload::draw(ctx, scene);
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
