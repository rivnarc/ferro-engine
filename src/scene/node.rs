use crate::scene::Transform;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PhysicsBody {
    pub velocity: Vec2,
    pub gravity: bool,
    pub grounded: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub name: String,
    pub transform: Transform,
    pub script: Option<String>,
    pub texture: Option<String>,
    pub collider: Option<Vec2>,
    pub physics: Option<PhysicsBody>,
    #[serde(skip)]
    pub script_initialized: bool,
    #[serde(skip)]
    pub previous_position: Vec2,
    pub alive: bool,
}
