use crate::scene::{Node, Scene, Transform};
use glam::Vec2;

pub fn draw(ctx: &egui::Context, scene: &mut Scene, selected: usize) -> Option<usize> {
    let mut new_selected = None;

    egui::Window::new("Hierarchy").show(ctx, |ui| {
        // Кнопка добавления узла
        if ui.button("+ Add Node").clicked() {
            // Позиция — центр камеры (чтобы новый узел был виден)
            let pos = scene.camera.position;
            let idx = scene.add_node(Node {
                name: format!("Node {}", scene.nodes.len()),
                transform: Transform {
                    position: pos,
                    scale: Vec2::splat(128.0), // дефолтный размер спрайта
                    ..Default::default()
                },
                ..Default::default()
            });
            new_selected = Some(idx);
        }

        ui.separator();

        // Список живых узлов
        for i in scene.alive_indices() {
            let node = &scene.nodes[i];
            let label = format!("{}: {}", i, node.name);
            if ui.selectable_label(selected == i, label).clicked() {
                new_selected = Some(i);
            }
        }
    });

    new_selected
}
