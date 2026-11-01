use crate::editor::Editor;
use crate::editor::export::ExportStatus;
use crate::scene::Scene;

pub fn draw(ctx: &egui::Context, scene: &mut Scene, editor: &mut Editor) {
    egui::Window::new("Save / Load").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Save As...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("RON Scene", &["ron"])
                    .set_file_name("scene.ron")
                    .save_file()
                {
                    let s = ron::ser::to_string_pretty(&*scene, Default::default()).unwrap();
                    if let Err(e) = std::fs::write(&path, s) {
                        eprintln!("Ошибка сохранения: {}", e);
                    }
                }
            }
            if ui.button("Load...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("RON Scene", &["ron"])
                    .pick_file()
                {
                    if let Ok(s) = std::fs::read_to_string(&path) {
                        if let Ok(new_scene) = ron::from_str(&s) {
                            *scene = new_scene;
                        }
                    }
                }
            }
        });

        ui.separator();

        ui.separator();

        ui.label("Export");

        match &editor.export_status {
            ExportStatus::Idle => {}
            ExportStatus::Building => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Сборка...");
                });
            }
            ExportStatus::Done(path) => {
                ui.label(format!("Готово: {:?}", path));
            }
            ExportStatus::Error(e) => {
                ui.colored_label(egui::Color32::RED, format!("Ошибка: {}", e));
            }
        }

        let can_export = matches!(
            editor.export_status,
            ExportStatus::Idle | ExportStatus::Done(_) | ExportStatus::Error(_)
        );
        ui.add_enabled_ui(can_export, |ui| {
            if ui.button("Export Game").clicked() {
                editor.export_status = ExportStatus::Building;
                editor.export_rx = Some(crate::editor::export::export_game_async(scene.clone()));
            }
        });
    });
}
