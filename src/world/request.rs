use bevy::math::I64Vec3;

#[derive(Debug)]
/// The vector provided in all of these enums are absolutely positioned
pub enum OrganismRequest {
    /// Asks to move up by this amount
    MoveBy(I64Vec3),
    ProduceFoodAround(I64Vec3),
    KillAround(I64Vec3),
    EatFoodAround(I64Vec3),
    Reproduce,
    Starve,
}
