use crate::editor::Editor;
use crate::scene::{Scene, Transform};

pub fn draw(ctx: &egui::Context, scene: &mut Scene, editor: &mut Editor) {
    egui::Window::new("Inspector").show(ctx, |ui| {
        if editor.selected.is_empty() {
            ui.label("No selection");
            return;
        }

        let count = editor.selected.len();
        if count > 1 {
            ui.label(format!("{} nodes selected", count));
            ui.separator();
        }

        let selected: Vec<usize> = editor
            .selected
            .iter()
            .copied()
            .filter(|&i| scene.nodes.get(i).map(|n| n.alive).unwrap_or(false))
            .collect();

        if selected.is_empty() {
            ui.label("No alive selection");
            return;
        }

        // --- Name (только для одного) ---
        if count == 1 {
            let idx = selected[0];
            let mut name = scene.nodes[idx].name.clone();
            ui.label("Name");
            if ui.text_edit_singleline(&mut name).lost_focus() {
                scene.rename_node(idx, &name);
            }
        }

        ui.separator();

        // --- Position ---
        ui.label("Position");
        ui.horizontal(|ui| {
            edit_axis(
                ui,
                scene,
                &selected,
                |t| t.position.x,
                |t, v| t.position.x = v,
                0.0,
            );
            edit_axis(
                ui,
                scene,
                &selected,
                |t| t.position.y,
                |t, v| t.position.y = v,
                0.0,
            );
        });

        ui.label("Rotation");
        let first_rot = scene.nodes[selected[0]].transform.rotation;
        let all_same_rot = selected
            .iter()
            .all(|&i| scene.nodes[i].transform.rotation == first_rot);
        let mut degrees = if all_same_rot {
            first_rot.to_degrees()
        } else {
            0.0
        };
        let response = ui.add(
            egui::Slider::new(&mut degrees, -180.0..=180.0)
                .suffix("°")
                .text(if all_same_rot { "" } else { "?" }),
        );
        if response.changed() {
            let delta = if all_same_rot {
                degrees.to_radians() - first_rot
            } else {
                degrees.to_radians()
            };
            for &i in &selected {
                scene.nodes[i].transform.rotation += delta;
            }
        }

        ui.label("Scale");
        ui.horizontal(|ui| {
            edit_axis(
                ui,
                scene,
                &selected,
                |t| t.scale.x,
                |t, v| t.scale.x = v,
                1.0,
            );
            edit_axis(
                ui,
                scene,
                &selected,
                |t| t.scale.y,
                |t, v| t.scale.y = v,
                1.0,
            );
        });

        ui.separator();

        // --- Delete ---
        if ui.button("Delete Selected").clicked() {
            for &i in &selected {
                scene.remove_node(i);
            }
            editor.selected.clear();
        }
    });
}

fn edit_axis(
    ui: &mut egui::Ui,
    scene: &mut Scene,
    selected: &[usize],
    get: impl Fn(&Transform) -> f32,
    set: impl Fn(&mut Transform, f32),
    baseline_if_mixed: f32,
) {
    let first = get(&scene.nodes[selected[0]].transform);
    let all_same = selected
        .iter()
        .all(|&i| get(&scene.nodes[i].transform) == first);

    let mut value = if all_same { first } else { baseline_if_mixed };

    let response = ui.add(
        egui::DragValue::new(&mut value)
            .speed(1.0)
            .prefix(if all_same { "" } else { "? " }),
    );

    if response.changed() {
        let delta = if all_same {
            value - first
        } else {
            value - baseline_if_mixed
        };
        for &i in selected {
            let old = get(&scene.nodes[i].transform);
            set(&mut scene.nodes[i].transform, old + delta);
        }
    }
}
