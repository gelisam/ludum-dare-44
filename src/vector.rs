use glam::f32::Vec2;

pub fn rotate(vector: Vec2, rotation: f32) -> Vec2 {
    let (sin, cos) = rotation.sin_cos();

    vector.x * Vec2::new(cos, sin) +
    vector.y * Vec2::new(-sin, cos)
}
