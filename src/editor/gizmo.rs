use crate::scene::Scene;
use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GizmoAxis {
    X,
    Y,
}

pub struct GizmoDrag {
    pub axis: GizmoAxis,
    pub start_mouse_world: Vec2,
    pub start_positions: Vec<(usize, Vec2)>,
}

impl GizmoDrag {
    pub fn new(axis: GizmoAxis, mouse_world: Vec2, scene: &Scene, selected: &[usize]) -> Self {
        let positions = selected
            .iter()
            .filter_map(|&i| {
                scene
                    .nodes
                    .get(i)
                    .filter(|n| n.alive)
                    .map(|n| (i, n.transform.position))
            })
            .collect();
        Self {
            axis,
            start_mouse_world: mouse_world,
            start_positions: positions,
        }
    }

    pub fn update(&self, scene: &mut Scene, mouse_world: Vec2) {
        let delta = mouse_world - self.start_mouse_world;
        for &(idx, start_pos) in &self.start_positions {
            if let Some(node) = scene.nodes.get_mut(idx) {
                let mut new_pos = start_pos;
                match self.axis {
                    GizmoAxis::X => new_pos.x += delta.x,
                    GizmoAxis::Y => new_pos.y += delta.y,
                }
                node.transform.position = new_pos;
            }
        }
    }
}

const GIZMO_LENGTH: f32 = 80.0;
const GIZMO_THICKNESS: f32 = 6.0;

pub fn draw_gizmo<F>(scene: &Scene, selected: &[usize], mut render_rect: F)
where
    F: FnMut(Vec2, Vec2, [f32; 4]),
{
    let Some(&first) = selected
        .iter()
        .find(|&&i| scene.nodes.get(i).map(|n| n.alive).unwrap_or(false))
    else {
        return;
    };
    let center = scene.world_matrix(first).transform_point2(Vec2::ZERO);

    let x_pos = center + Vec2::new(GIZMO_LENGTH * 0.5, 0.0);
    let x_size = Vec2::new(GIZMO_LENGTH, GIZMO_THICKNESS);
    render_rect(x_pos, x_size, [1.0, 0.2, 0.2, 1.0]);

    let x_tip_pos = center + Vec2::new(GIZMO_LENGTH, 0.0);
    let x_tip_size = Vec2::splat(GIZMO_THICKNESS * 2.0);
    render_rect(x_tip_pos, x_tip_size, [1.0, 0.2, 0.2, 1.0]);

    let y_pos = center + Vec2::new(0.0, GIZMO_LENGTH * 0.5);
    let y_size = Vec2::new(GIZMO_THICKNESS, GIZMO_LENGTH);
    render_rect(y_pos, y_size, [0.2, 1.0, 0.2, 1.0]);

    let y_tip_pos = center + Vec2::new(0.0, GIZMO_LENGTH);
    let y_tip_size = Vec2::splat(GIZMO_THICKNESS * 2.0);
    render_rect(y_tip_pos, y_tip_size, [0.2, 1.0, 0.2, 1.0]);
}

pub fn hit_test_gizmo(scene: &Scene, selected: &[usize], mouse_world: Vec2) -> Option<GizmoAxis> {
    let Some(&first) = selected
        .iter()
        .find(|&&i| scene.nodes.get(i).map(|n| n.alive).unwrap_or(false))
    else {
        return None;
    };
    let center = scene.world_matrix(first).transform_point2(Vec2::ZERO);

    let x_min = center.x - GIZMO_THICKNESS;
    let x_max = center.x + GIZMO_LENGTH + GIZMO_THICKNESS;
    let x_y_min = center.y - GIZMO_THICKNESS;
    let x_y_max = center.y + GIZMO_THICKNESS;
    if mouse_world.x >= x_min
        && mouse_world.x <= x_max
        && mouse_world.y >= x_y_min
        && mouse_world.y <= x_y_max
    {
        return Some(GizmoAxis::X);
    }

    let y_min = center.y - GIZMO_THICKNESS;
    let y_max = center.y + GIZMO_LENGTH + GIZMO_THICKNESS;
    let y_x_min = center.x - GIZMO_THICKNESS;
    let y_x_max = center.x + GIZMO_THICKNESS;
    if mouse_world.x >= y_x_min
        && mouse_world.x <= y_x_max
        && mouse_world.y >= y_min
        && mouse_world.y <= y_max
    {
        return Some(GizmoAxis::Y);
    }

    None
}
