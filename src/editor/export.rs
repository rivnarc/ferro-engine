use crate::scene::Scene;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::{Receiver, channel};

#[derive(Clone, Debug)]
pub enum ExportStatus {
    Idle,
    Building,
    Done(PathBuf),
    Error(String),
}

pub fn export_game_async(scene: Scene) -> Receiver<Result<PathBuf, String>> {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let result = export_game(&scene);
        let _ = tx.send(result);
    });
    rx
}

pub fn export_game(scene: &Scene) -> Result<PathBuf, String> {
    let scene_path = "assets/scenes/export.ron";
    std::fs::create_dir_all("assets/scenes")
        .map_err(|e| format!("Не удалось создать assets/scenes: {}", e))?;
    let ron_str = ron::ser::to_string_pretty(scene, Default::default())
        .map_err(|e| format!("Ошибка сериализации сцены: {}", e))?;
    std::fs::write(scene_path, ron_str).map_err(|e| format!("Не удалось записать сцену: {}", e))?;

    let output = Command::new("cargo")
        .args(["build", "--release", "--no-default-features"])
        .output()
        .map_err(|e| format!("Не удалось запустить cargo: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Ошибка сборки:\n{}", stderr));
    }

    let exe_path = PathBuf::from("target/release/ferro-engine");
    let exe_dir = exe_path.parent().ok_or("Не удалось найти папку .exe")?;
    let assets_dest = exe_dir.join("assets");

    if assets_dest.exists() {
        std::fs::remove_dir_all(&assets_dest)
            .map_err(|e| format!("Не удалось удалить старые assets: {}", e))?;
    }
    copy_dir_recursive("assets", &assets_dest)
        .map_err(|e| format!("Не удалось скопировать assets: {}", e))?;

    Ok(exe_path)
}

fn copy_dir_recursive(src: &str, dst: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path.to_string_lossy(), &dest)?;
        } else {
            std::fs::copy(&path, &dest)?;
        }
    }
    Ok(())
}
