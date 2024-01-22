use bevy::math::I64Vec2;

use crate::Direction;

#[derive(Debug)]
/// The vector provided in all of these enums are absolutely positioned
pub enum OrganismRequest {
    /// Asks to move up by this amount
    MoveBy(I64Vec2),
    IntelligentMove(Vec<(I64Vec2, Direction)>),
    ProduceFoodAround(I64Vec2),
    KillAround(I64Vec2),
    EatFoodAround(I64Vec2),
    Reproduce,
    Starve,
}
