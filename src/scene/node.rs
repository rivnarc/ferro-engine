use crate::scene::Transform;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub transform: Transform,
    pub alive: bool,
}
