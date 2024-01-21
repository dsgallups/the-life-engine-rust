use bevy::math::I64Vec2;

#[derive(Debug)]
/// The vector provided in all of these enums are absolutely positioned
pub enum OrganismRequest {
    /// Asks to move up by this amount
    MoveBy(I64Vec2),
    ProduceFoodAround(I64Vec2),
    KillAround(I64Vec2),
    EatFoodAround(I64Vec2),
    Reproduce,
    Starve,
}
