use bevy::math::I64Vec3;

#[derive(Debug)]
pub enum WorldRequest {
    /// Asks to move up by this amount
    MoveBy(I64Vec3),
    Food(I64Vec3),
    Kill(I64Vec3),
    EatFood(I64Vec3),
    Reproduce,
    Starve,
}
