use crate::scene::Scene;

pub fn draw(ctx: &egui::Context, scene: &mut Scene, selected: usize) {
    if !scene.nodes.get(selected).map(|n| n.alive).unwrap_or(false) {
        return;
    }

    egui::Window::new("Inspector").show(ctx, |ui| {
        let node = &mut scene.nodes[selected];

        ui.label("Name");
        ui.text_edit_singleline(&mut node.name);

        ui.separator();
        ui.label("Position");
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut node.transform.position.x).speed(1.0));
            ui.add(egui::DragValue::new(&mut node.transform.position.y).speed(1.0));
        });

        ui.label("Rotation");
        ui.add(egui::Slider::new(
            &mut node.transform.rotation,
            -3.14..=3.14,
        ));

        ui.label("Scale");
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut node.transform.scale.x).speed(1.0));
            ui.add(egui::DragValue::new(&mut node.transform.scale.y).speed(1.0));
        });

        ui.separator();

        if ui.button("Delete Node").clicked() {
            scene.remove_node(selected);
        }
    });
}
