use crate::engine::FpsCounter;
use crate::engine::{EditorState, RenderState};
use glow::HasContext;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowId;

pub struct App {
    render: RenderState,
    editor: EditorState,
    fps: FpsCounter,
    show_colliders: bool,
    is_editor: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            render: RenderState::empty(),
            editor: EditorState::default(),
            fps: FpsCounter::default(),
            show_colliders: true,
            is_editor: true,
        }
    }
}

impl App {
    fn view_proj(&self) -> glam::Mat3 {
        let size = self.render.viewport_size();
        self.editor.scene.camera.view_proj(size.x, size.y)
    }

    #[cfg(feature = "editor")]
    fn handle_click(&mut self) {
        if !self.editor.input.take_click() {
            return;
        }
        let vp_size = self.render.viewport_size();
        let view_proj = self.view_proj();
        let mouse_screen = self.editor.input.mouse_position();
        let mouse_world = crate::math::screen_to_world(mouse_screen, vp_size, view_proj);

        if let Some(axis) = crate::editor::gizmo::hit_test_gizmo(
            &self.editor.scene,
            &self.editor.editor.selected,
            mouse_world,
        ) {
            self.editor.editor.drag = Some(crate::editor::gizmo::GizmoDrag::new(
                axis,
                mouse_world,
                &self.editor.scene,
                &self.editor.editor.selected,
            ));
            return;
        }

        let hits = crate::editor::viewport::hit_test_all(
            &self.editor.scene,
            mouse_screen,
            vp_size,
            view_proj,
        );
        let ctrl = self.editor.input.ctrl;
        let shift = self.editor.input.shift;

        if let Some(&top) = hits.first() {
            if ctrl {
                self.editor.editor.toggle(top);
            } else if shift {
                self.editor.editor.add(top);
            } else {
                self.editor.editor.select_single(top);
            }
        } else if !ctrl && !shift {
            self.editor.editor.selected.clear();
        }
    }

    #[cfg(feature = "editor")]
    fn update_drag(&mut self) {
        let Some(drag) = &self.editor.editor.drag else {
            return;
        };
        if !self.editor.input.mouse_down {
            self.editor.editor.drag = None;
            return;
        }
        let vp_size = self.render.viewport_size();
        let view_proj = self.view_proj();
        let mouse_screen = self.editor.input.mouse_position();
        let mouse_world = crate::math::screen_to_world(mouse_screen, vp_size, view_proj);
        drag.update(&mut self.editor.scene, mouse_world);
    }

    #[cfg(feature = "editor")]
    pub fn new_editor() -> Self {
        Self {
            render: RenderState::empty(),
            editor: EditorState::default(),
            fps: FpsCounter::default(),
            show_colliders: false,
            is_editor: true,
        }
    }

    pub fn new_game(scene_path: String) -> Self {
        let scene = std::fs::read_to_string(&scene_path)
            .ok()
            .and_then(|s| ron::from_str::<crate::scene::Scene>(&s).ok())
            .unwrap_or_default();

        let mut editor_state = EditorState::default();
        editor_state.scene = scene;

        Self {
            render: RenderState::empty(),
            editor: editor_state,
            fps: FpsCounter::default(),
            show_colliders: false,
            is_editor: false,
        }
    }

    fn render(&self) {
        let (Some(g), Some(r)) = (&self.render.graphics, &self.render.renderer) else {
            return;
        };
        let view_proj = self.view_proj();

        unsafe {
            g.gl.clear_color(0.1, 0.1, 0.15, 1.0);
            g.gl.clear(glow::COLOR_BUFFER_BIT);
        }

        r.render_scene(&g.gl, &self.editor.scene, view_proj);

        #[cfg(feature = "editor")]
        for &idx in &self.editor.editor.selected {
            if self
                .editor
                .scene
                .nodes
                .get(idx)
                .map(|n| n.alive)
                .unwrap_or(false)
            {
                r.draw_selection_outline(&g.gl, &self.editor.scene, idx, view_proj);
            }
        }

        if self.show_colliders {
            for i in 0..self.editor.scene.nodes.len() {
                r.draw_collider(&g.gl, &self.editor.scene, i, view_proj);
            }
        }

        #[cfg(feature = "editor")]
        if !self.editor.editor.selected.is_empty() {
            crate::editor::gizmo::draw_gizmo(
                &self.editor.scene,
                &self.editor.editor.selected,
                |pos, size, color| r.draw_world_rect(&g.gl, pos, size, view_proj, color),
            );
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.render.window.is_some() {
            return;
        }
        self.render = RenderState::new(event_loop, "Ferro Engine", self.is_editor);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        #[cfg(feature = "editor")]
        if self.is_editor {
            if !matches!(event, WindowEvent::RedrawRequested) {
                if let (Some(gui), Some(win)) = (&mut self.render.gui, &self.render.window) {
                    if gui.on_event(&win.window, &event) {
                        return;
                    }
                }
            }
        }

        if let Some(code) = self.editor.input.process_event(&event) {
            if code == KeyCode::Escape {
                event_loop.exit();
                return;
            }
            #[cfg(feature = "editor")]
            if self.is_editor {
                self.editor.editor.on_key(code, &mut self.editor.scene);
            }
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                self.render.resize(size.width, size.height);
            }

            WindowEvent::RedrawRequested => {
                self.editor.time.update();
                self.fps.update();

                #[cfg(feature = "editor")]
                if self.is_editor {
                    let egui_wants_kb = self.render.wants_keyboard();
                    crate::editor::viewport::update_camera(
                        &mut self.editor.scene.camera,
                        &self.editor.input,
                        self.editor.time.delta,
                    );
                    let scroll = self.editor.input.take_scroll();
                    if scroll != 0.0 && !egui_wants_kb {
                        crate::editor::viewport::zoom_camera(&mut self.editor.scene.camera, scroll);
                    }
                }

                #[cfg(feature = "editor")]
                if self.is_editor {
                    self.editor.editor.update(
                        &self.editor.input,
                        &self.editor.time,
                        &mut self.editor.scene,
                    );
                    self.handle_click();
                    self.update_drag();
                }

                self.editor.script_engine.update(
                    &mut self.editor.scene,
                    &self.editor.input,
                    self.editor.time.delta,
                    self.fps.fps,
                );

                #[cfg(feature = "editor")]
                if let Some(rx) = &self.editor.editor.export_rx {
                    match rx.try_recv() {
                        Ok(Ok(path)) => {
                            self.editor.editor.export_status =
                                crate::editor::export::ExportStatus::Done(path);
                            self.editor.editor.export_rx = None;
                        }
                        Ok(Err(e)) => {
                            self.editor.editor.export_status =
                                crate::editor::export::ExportStatus::Error(e);
                            self.editor.editor.export_rx = None;
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => {}
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                            self.editor.editor.export_status =
                                crate::editor::export::ExportStatus::Error(
                                    "Поток завершился".to_string(),
                                );
                            self.editor.editor.export_rx = None;
                        }
                    }
                }

                crate::physics::integrate_and_resolve(
                    &mut self.editor.scene,
                    self.editor.time.delta,
                );

                self.render();

                #[cfg(feature = "editor")]
                if self.is_editor {
                    if let (Some(gui), Some(win)) = (&mut self.render.gui, &self.render.window) {
                        let scene = &mut self.editor.scene;
                        let editor = &mut self.editor.editor;
                        let mut show_colliders = self.show_colliders;
                        let fps_value = self.fps.fps;
                        gui.run_ui(&win.window, |ctx| {
                            crate::editor::hierarchy::draw(ctx, scene, editor);
                            crate::editor::inspector::draw(ctx, scene, editor);
                            crate::editor::savenload::draw(ctx, scene, editor);
                            crate::editor::debug::draw(ctx, fps_value, &mut show_colliders);
                        });
                        self.show_colliders = show_colliders;
                        gui.paint(&win.window);
                    }
                }

                self.render.swap_buffers();
                self.render.request_redraw();
            }
            _ => (),
        }
    }
}
