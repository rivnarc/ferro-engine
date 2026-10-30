use crate::editor::Editor;
use crate::scene::{Node, Scene, Transform};
use glam::Vec2;

pub fn draw(ctx: &egui::Context, scene: &mut Scene, editor: &mut Editor) {
    egui::Window::new("Hierarchy").show(ctx, |ui| {
        if ui.button("+ Add Node").clicked() {
            let pos = scene.camera.position;
            let idx = scene.add_node(Node {
                name: String::new(),
                transform: Transform {
                    position: pos,
                    scale: Vec2::splat(256.0),
                    ..Default::default()
                },
                ..Default::default()
            });
            editor.select_single(idx);
        }

        ui.separator();

        for i in scene.alive_indices() {
            let node = &scene.nodes[i];
            let label = format!("{}: {}", i, node.name);
            let selected = editor.is_selected(i);

            // egui: клик с модификаторами
            let response = ui.selectable_label(selected, label);
            if response.clicked() {
                let mods = ui.input(|i| i.modifiers);
                if mods.ctrl || mods.command {
                    editor.toggle(i);
                } else if mods.shift {
                    editor.add(i);
                } else {
                    editor.select_single(i);
                }
            }
        }
    });
}
