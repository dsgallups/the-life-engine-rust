use bevy::math::I64Vec3;

#[derive(Debug)]
pub enum WorldRequest {
    /// Asks to move up by this amount
    MoveBy(I64Vec3),
    Food(I64Vec3),
    Kill(Uuid),
    EatFood(I64Vec3),
}
