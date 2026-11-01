use crate::scene::Scene;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use rhai::{AST, Engine, Scope};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, channel};
use std::time::SystemTime;
use winit::keyboard::KeyCode;

pub struct ScriptEngine {
    engine: Engine,
    cache: HashMap<String, (SystemTime, AST)>,
    _watcher: RecommendedWatcher,
    events: Receiver<notify::Result<Event>>,
    total_time: f64,
    last_logs: HashMap<String, f64>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
        .expect("watcher");

        std::fs::create_dir_all("assets/scripts").ok();

        watcher
            .watch(
                std::path::Path::new("assets/scripts"),
                RecursiveMode::NonRecursive,
            )
            .expect("watch assets/scripts");

        let mut engine = Engine::new();
        engine.register_fn("log", |msg: &str| {
            println!("[SCRIPT] {}", msg);
        });

        engine.register_fn("sin", |x: f64| x.sin());
        engine.register_fn("cos", |x: f64| x.cos());
        engine.register_fn("tan", |x: f64| x.tan());
        engine.register_fn("sqrt", |x: f64| x.sqrt());
        engine.register_fn("abs", |x: f64| x.abs());
        engine.register_fn("floor", |x: f64| x.floor());
        engine.register_fn("ceil", |x: f64| x.ceil());
        engine.register_fn("round", |x: f64| x.round());
        engine.register_fn("min", |a: f64, b: f64| a.min(b));
        engine.register_fn("max", |a: f64, b: f64| a.max(b));
        engine.register_fn("pi", || std::f64::consts::PI);

        Self {
            engine,
            cache: HashMap::new(),
            _watcher: watcher,
            events: rx,
            total_time: 0.0,
            last_logs: HashMap::new(),
        }
    }

    fn drain_events(&mut self, scene: &mut Scene) {
        while let Ok(Ok(event)) = self.events.try_recv() {
            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {}
                _ => continue,
            }

            for path in event.paths {
                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    continue;
                };
                if !name.ends_with(".rhai") {
                    continue;
                }
                if !path.exists() {
                    continue;
                }

                let mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
                if let Some((cached_mtime, _)) = self.cache.get(name) {
                    if Some(*cached_mtime) == mtime {
                        continue;
                    }
                }

                self.cache.remove(name);
                for node in &mut scene.nodes {
                    if node.script.as_deref() == Some(name) {
                        node.script_initialized = false;
                    }
                }
            }
        }
    }

    fn load_ast(&mut self, script_name: &str) -> Option<AST> {
        let path = format!("assets/scripts/{}", script_name);

        let mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok()?;

        if let Some((cached_mtime, ast)) = self.cache.get(script_name) {
            if *cached_mtime == mtime {
                return Some(ast.clone());
            }
        }

        let source = std::fs::read_to_string(&path).ok()?;
        let ast = self.engine.compile(&source).ok()?;
        self.cache
            .insert(script_name.to_string(), (mtime, ast.clone()));
        Some(ast)
    }

    pub fn update(&mut self, scene: &mut Scene, input: &crate::engine::Input, dt: f32, fps: f32) {
        self.drain_events(scene);
        self.total_time += dt as f64;

        let collision_map: Vec<Vec<usize>> = (0..scene.nodes.len())
            .map(|i| crate::physics::collisions_for(scene, i))
            .collect();

        for (i, node) in scene.nodes.iter_mut().enumerate() {
            if !node.alive {
                continue;
            }
            let Some(script_name) = node.script.clone() else {
                continue;
            };

            let Some(ast) = self.load_ast(&script_name) else {
                eprintln!("Не найден скрипт: {}", script_name);
                continue;
            };

            let mut scope = Scope::new();
            let last_log = *self.last_logs.get(&script_name).unwrap_or(&0.0);
            scope.push("last_log", last_log);
            scope.push("current_rotation", node.transform.rotation as f64);
            scope.push("pos_x", node.transform.position.x as f64);
            scope.push("pos_y", node.transform.position.y as f64);
            scope.push("scale_x", node.transform.scale.x as f64);
            scope.push("scale_y", node.transform.scale.y as f64);
            scope.push("dt", dt as f64);
            scope.push("time", self.total_time);
            scope.push("key_w", input.is_pressed(KeyCode::KeyW));
            scope.push("key_a", input.is_pressed(KeyCode::KeyA));
            scope.push("key_s", input.is_pressed(KeyCode::KeyS));
            scope.push("key_d", input.is_pressed(KeyCode::KeyD));
            scope.push("up", input.is_pressed(KeyCode::ArrowUp));
            scope.push("down", input.is_pressed(KeyCode::ArrowDown));
            scope.push("left", input.is_pressed(KeyCode::ArrowLeft));
            scope.push("right", input.is_pressed(KeyCode::ArrowRight));
            scope.push("fps", fps as f64);
            scope.push("colliding", !collision_map[i].is_empty());
            scope.push("collision_count", collision_map[i].len() as i64);

            if let Some(body) = &node.physics {
                scope.push("vel_x", body.velocity.x as f64);
                scope.push("vel_y", body.velocity.y as f64);
                scope.push("grounded", body.grounded);
            } else {
                scope.push("vel_x", 0.0);
                scope.push("vel_y", 0.0);
                scope.push("grounded", false);
            }

            if let Err(e) = self.engine.run_ast_with_scope(&mut scope, &ast) {
                eprintln!("Ошибка скрипта {}: {}", script_name, e);
                continue;
            }

            if !node.script_initialized {
                if ast.iter_functions().any(|f| f.name == "ready") {
                    if let Err(e) = self.engine.call_fn::<()>(&mut scope, &ast, "ready", ()) {
                        eprintln!("Ошибка ready() в {}: {}", script_name, e);
                    }
                }
                node.script_initialized = true;
            }

            if ast.iter_functions().any(|f| f.name == "update") {
                if let Err(e) = self.engine.call_fn::<()>(&mut scope, &ast, "update", ()) {
                    eprintln!("Ошибка update() в {}: {}", script_name, e);
                }
            }

            if let Some(v) = scope.get_value::<f64>("current_rotation") {
                node.transform.rotation = v as f32;
            }
            if let Some(v) = scope.get_value::<f64>("pos_x") {
                node.transform.position.x = v as f32;
            }
            if let Some(v) = scope.get_value::<f64>("pos_y") {
                node.transform.position.y = v as f32;
            }
            if let Some(v) = scope.get_value::<f64>("scale_x") {
                node.transform.scale.x = v as f32;
            }
            if let Some(v) = scope.get_value::<f64>("scale_y") {
                node.transform.scale.y = v as f32;
            }
            if let Some(body) = &mut node.physics {
                if let Some(v) = scope.get_value::<f64>("vel_x") {
                    body.velocity.x = v as f32;
                }
                if let Some(v) = scope.get_value::<f64>("vel_y") {
                    body.velocity.y = v as f32;
                }
            }
            if let Some(v) = scope.get_value::<f64>("last_log") {
                self.last_logs.insert(script_name.clone(), v);
            }
        }
    }
}
