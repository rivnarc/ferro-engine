pub use glam::{Mat3, Vec2};

pub fn ortho_2d(center: Vec2, half_size: Vec2) -> Mat3 {
    let translate = Mat3::from_translation(-center);
    let scale = Mat3::from_scale(Vec2::new(1.0 / half_size.x, -1.0 / half_size.y));
    scale * translate
}

pub fn screen_to_ndc(screen: Vec2, viewport: Vec2) -> Vec2 {
    Vec2::new(
        (screen.x / viewport.x) * 2.0 - 1.0,
        1.0 - (screen.y / viewport.y) * 2.0,
    )
}

pub fn ndc_to_world(ndc: Vec2, view_proj: Mat3) -> Vec2 {
    view_proj.inverse().transform_point2(ndc)
}

pub fn screen_to_world(screen: Vec2, viewport: Vec2, view_proj: Mat3) -> Vec2 {
    ndc_to_world(screen_to_ndc(screen, viewport), view_proj)
}
