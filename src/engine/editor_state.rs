use crate::editor::Editor;
use crate::engine::{Input, ScriptEngine, Time};
use crate::scene::Scene;
use glam::Vec2;

pub struct EditorState {
    pub scene: Scene,
    pub editor: Editor,
    pub input: Input,
    pub time: Time,
    pub script_engine: ScriptEngine,
}

impl Default for EditorState {
    fn default() -> Self {
        let mut scene = Scene::default();
        scene.add_node(crate::scene::Node {
            name: "Node".to_string(),
            transform: crate::scene::Transform {
                position: Vec2::new(0.0, 0.0),
                scale: Vec2::splat(256.0),
                ..Default::default()
            },
            ..Default::default()
        });

        Self {
            scene,
            editor: Editor::default(),
            input: Input::default(),
            time: Time::new(),
            script_engine: ScriptEngine::new(),
        }
    }
}

impl EditorState {
    pub fn _update(&mut self, dt: f32, _viewport_size: Vec2) {
        self.time.update();
        self.time.delta = dt;
        crate::editor::viewport::update_camera(
            &mut self.scene.camera,
            &self.input,
            self.time.delta,
        );

        self.editor.update(&self.input, &self.time, &mut self.scene);

        self.script_engine
            .update(&mut self.scene, &self.input, self.time.delta);
    }
}
