use crate::scene::Transform;

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub name: String,
    pub transform: Transform,
    pub alive: bool,
}
