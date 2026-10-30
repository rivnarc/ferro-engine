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
        let base = if node.name.is_empty() {
            "Node".to_string()
        } else {
            node.name.clone()
        };
        node.name = self.unique_name(&base);
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

    pub fn rename_node(&mut self, idx: usize, new_name: &str) {
        if idx >= self.nodes.len() || !self.nodes[idx].alive {
            return;
        }
        let mut name = new_name.to_string();
        let mut counter = 1;
        while self
            .nodes
            .iter()
            .enumerate()
            .any(|(i, n)| i != idx && n.alive && n.name == name)
        {
            name = format!("{} ({})", new_name, counter);
            counter += 1;
        }
        self.nodes[idx].name = name;
    }

    pub fn world_matrix(&self, index: usize) -> Mat3 {
        self.nodes[index].transform.to_matrix()
    }
    fn unique_name(&self, base: &str) -> String {
        let mut name = base.to_string();
        let mut counter = 1;
        while self.nodes.iter().any(|n| n.alive && n.name == name) {
            name = format!("{} ({})", base, counter);
            counter += 1;
        }
        name
    }
}
