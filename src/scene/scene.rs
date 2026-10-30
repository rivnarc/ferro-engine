use crate::scene::{Camera2D, Node};
use glam::Mat3;

#[derive(Default)]
pub struct Scene {
    pub nodes: Vec<Node>,
    pub camera: Camera2D,
}

impl Scene {
    pub fn add_node(&mut self, mut node: Node) -> usize {
        node.alive = true;
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn remove_node(&mut self, idx: usize) {
        if idx >= self.nodes.len() || !self.nodes[idx].alive {
            return;
        }
        self.nodes[idx].alive = false;
    }

    pub fn alive_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.alive)
            .map(|(i, _)| i)
    }

    pub fn world_matrix(&self, index: usize) -> Mat3 {
        self.nodes[index].transform.to_matrix()
    }
}
