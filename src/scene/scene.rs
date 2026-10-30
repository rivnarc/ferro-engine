use crate::scene::Node;
use glam::Mat3;

#[derive(Default)]
pub struct Scene {
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn add_node(&mut self, node: Node) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn attach(&mut self, child: usize, parent: usize) {
        if child == parent || self.is_ancestor(child, parent) {
            return;
        }

        if let Some(old_parent) = self.nodes[child].parent {
            if let Some(p) = self.nodes.get_mut(old_parent) {
                p.children.retain(|&c| c != child);
            }
        }

        self.nodes[child].parent = Some(parent);
        if let Some(p) = self.nodes.get_mut(parent) {
            p.children.push(child);
        }
    }

    pub fn detach(&mut self, child: usize) {
        if let Some(parent) = self.nodes[child].parent.take() {
            if let Some(p) = self.nodes.get_mut(parent) {
                p.children.retain(|&c| c != child);
            }
        }
    }

    fn is_ancestor(&self, a: usize, b: usize) -> bool {
        let mut current = Some(b);
        while let Some(idx) = current {
            if idx == a {
                return true;
            }
            current = self.nodes[idx].parent;
        }
        false
    }

    pub fn world_matrix(&self, index: usize) -> Mat3 {
        let node = &self.nodes[index];
        let local = node.transform.to_matrix();
        match node.parent {
            Some(p) => self.world_matrix(p) * local,
            None => local,
        }
    }
}
