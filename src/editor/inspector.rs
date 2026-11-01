use crate::editor::Editor;
use crate::scene::{Scene, Transform};
use glam::Vec2;

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

        if count == 1 {
            let idx = selected[0];
            let mut name = scene.nodes[idx].name.clone();
            ui.label("Name");
            let response = ui.text_edit_singleline(&mut name);
            if response.changed() {
                scene.nodes[idx].name = name;
            }
            if response.lost_focus() {
                scene.rename_node(idx, &scene.nodes[idx].name.clone());
            }
        }

        ui.label("Script");
        ui.horizontal(|ui| {
            let current = scene.nodes[selected[0]].script.clone();
            ui.label(match &current {
                Some(name) => name.as_str(),
                None => "(none)",
            });

            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Rhai Script", &["rhai"])
                    .set_directory("assets/scripts")
                    .pick_file()
                {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let name = name.to_string();
                        for &i in &selected {
                            scene.nodes[i].script = Some(name.clone());
                        }
                    }
                }
            }

            if ui.button("New...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Rhai Script", &["rhai"])
                    .set_file_name("new_script.rhai")
                    .set_directory("assets/scripts")
                    .save_file()
                {
                    let template = r#"fn ready() {
    // One call code
}

fn update() {
    // Every frame call code
}"#;
                    if let Err(e) = std::fs::write(&path, template) {
                        eprintln!("Ошибка создания скрипта: {}", e);
                    } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let name = name.to_string();
                        for &i in &selected {
                            scene.nodes[i].script = Some(name.clone());
                            scene.nodes[i].script_initialized = false;
                        }
                    }
                }
            }

            if ui.button("Clear").clicked() {
                for &i in &selected {
                    scene.nodes[i].script = None;
                    scene.nodes[i].script_initialized = false;
                }
            }
        });

        ui.label("Texture");
        ui.horizontal(|ui| {
            let current = scene.nodes[selected[0]].texture.clone();
            ui.label(match &current {
                Some(name) => name.as_str(),
                None => "(none)",
            });

            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg"])
                    .set_directory("assets/textures")
                    .pick_file()
                {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        let name = name.to_string();
                        let dest = format!("assets/textures/{}", name);
                        if !std::path::Path::new(&dest).exists() {
                            if let Err(e) = std::fs::copy(&path, &dest) {
                                eprintln!("Ошибка копирования текстуры: {}", e);
                            }
                        }
                        for &i in &selected {
                            scene.nodes[i].texture = Some(name.clone());
                        }
                    }
                }
            }

            if ui.button("Clear").clicked() {
                for &i in &selected {
                    scene.nodes[i].texture = None;
                }
            }
        });

        ui.separator();

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

        ui.label("Collider");
        ui.horizontal(|ui| {
            let mut has_collider = scene.nodes[selected[0]].collider.is_some();
            if ui.checkbox(&mut has_collider, "Has collider").changed() {
                for &i in &selected {
                    scene.nodes[i].collider = if has_collider {
                        Some(Vec2::splat(64.0))
                    } else {
                        None
                    };
                }
            }
            if let Some(collider) = &mut scene.nodes[selected[0]].collider {
                let mut x = collider.x;
                let mut y = collider.y;
                if ui.add(egui::DragValue::new(&mut x).speed(1.0)).changed() {
                    for &i in &selected {
                        if let Some(c) = &mut scene.nodes[i].collider {
                            c.x = x;
                        }
                    }
                }
                if ui.add(egui::DragValue::new(&mut y).speed(1.0)).changed() {
                    for &i in &selected {
                        if let Some(c) = &mut scene.nodes[i].collider {
                            c.y = y;
                        }
                    }
                }
            }
        });

        ui.separator();
        ui.label("Physics");
        let mut is_dynamic = scene.nodes[selected[0]].physics.is_some();
        if ui.checkbox(&mut is_dynamic, "Entity (dynamic)").changed() {
            for &i in &selected {
                scene.nodes[i].physics = if is_dynamic {
                    Some(crate::scene::node::PhysicsBody::default())
                } else {
                    None
                };
            }
        }
        if let Some(body) = &mut scene.nodes[selected[0]].physics {
            ui.label("Velocity");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut body.velocity.x).speed(1.0));
                ui.add(egui::DragValue::new(&mut body.velocity.y).speed(1.0));
            });
            ui.checkbox(&mut body.gravity, "Gravity");
            ui.checkbox(&mut body.grounded, "Grounded");
        }

        ui.separator();

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
