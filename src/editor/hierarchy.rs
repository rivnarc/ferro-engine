use crate::scene::Scene;

pub fn draw(ctx: &egui::Context, scene: &Scene, selected: usize) -> Option<usize> {
    let mut new_selected = None;

    egui::Window::new("Hierarchy").show(ctx, |ui| {
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
