#[cfg(feature = "editor")]
use crate::editor::Editor;

use crate::engine::{Input, ScriptEngine, Time};
use crate::scene::Scene;
use glam::Vec2;

pub struct EditorState {
    pub scene: Scene,
    #[cfg(feature = "editor")]
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
            #[cfg(feature = "editor")]
            editor: Editor::default(),
            input: Input::default(),
            time: Time::new(),
            script_engine: ScriptEngine::new(),
        }
    }
}
