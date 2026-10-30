use crate::scene::Transform;

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub name: String,
    pub transform: Transform,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub alive: bool,
}
