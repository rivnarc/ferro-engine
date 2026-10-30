use crate::scene::Scene;

pub fn draw(ctx: &egui::Context, scene: &mut Scene, selected: usize) {
    if !scene.nodes.get(selected).map(|n| n.alive).unwrap_or(false) {
        return;
    }

    egui::Window::new("Inspector").show(ctx, |ui| {
        {
            let node = &mut scene.nodes[selected];
            ui.label("Name");
            ui.text_edit_singleline(&mut node.name);

            ui.separator();
            ui.label("Position");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut node.transform.position.x).speed(0.01));
                ui.add(egui::DragValue::new(&mut node.transform.position.y).speed(0.01));
            });

            ui.label("Rotation");
            ui.add(egui::Slider::new(
                &mut node.transform.rotation,
                -3.14..=3.14,
            ));

            ui.label("Scale");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut node.transform.scale.x).speed(0.01));
                ui.add(egui::DragValue::new(&mut node.transform.scale.y).speed(0.01));
            });
        }

        ui.separator();

        let current_parent = scene.nodes[selected].parent;
        let mut chosen_parent: Option<Option<usize>> = None;

        egui::ComboBox::from_id_salt("parent_combo")
            .selected_text(match current_parent {
                Some(p) => scene.nodes[p].name.clone(),
                None => "(none)".to_string(),
            })
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(current_parent.is_none(), "(none)")
                    .clicked()
                {
                    chosen_parent = Some(None);
                }
                for i in scene.alive_indices() {
                    if i == selected {
                        continue;
                    }
                    if scene.is_ancestor(selected, i) {
                        continue;
                    }
                    let label = format!("{}: {}", i, scene.nodes[i].name);
                    if ui
                        .selectable_label(current_parent == Some(i), label)
                        .clicked()
                    {
                        chosen_parent = Some(Some(i));
                    }
                }
            });

        if let Some(new_parent) = chosen_parent {
            match new_parent {
                Some(p) => scene.attach(selected, p),
                None => scene.detach(selected),
            }
        }

        ui.separator();

        if ui.button("Delete Node").clicked() {
            scene.remove_node(selected);
        }
    });
}
