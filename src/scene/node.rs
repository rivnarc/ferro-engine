use crate::scene::Transform;

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub _name: String,
    pub transform: Transform,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}
