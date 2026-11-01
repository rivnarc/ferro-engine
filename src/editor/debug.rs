pub fn draw(ctx: &egui::Context, fps: f32, show_colliders: &mut bool) {
    egui::Window::new("Debug")
        .resizable(false)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.label(format!("FPS: {:.1}", fps));
            ui.separator();
            ui.checkbox(show_colliders, "Show colliders");
        });
}
