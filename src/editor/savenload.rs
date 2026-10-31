use crate::scene::Scene;

pub fn draw(ctx: &egui::Context, scene: &mut Scene) {
    egui::Window::new("Save / Load").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
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

            if ui.button("Load").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("RON Scene", &["ron"])
                    .pick_file()
                {
                    match std::fs::read_to_string(&path) {
                        Ok(s) => match ron::from_str(&s) {
                            Ok(new_scene) => {
                                *scene = new_scene;
                            }
                            Err(e) => eprintln!("Ошибка парсинга RON: {}", e),
                        },
                        Err(e) => eprintln!("Ошибка чтения файла: {}", e),
                    }
                }
            }
        });
    });
}
