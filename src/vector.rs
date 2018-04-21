use ggez::graphics::Vector2;

pub fn rotate(vector: Vector2, rotation: f32) -> Vector2 {
    let (sin, cos) = rotation.sin_cos();

    vector.x * Vector2::new(cos, sin) +
    vector.y * Vector2::new(-sin, cos)
}
