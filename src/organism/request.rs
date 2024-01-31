use bevy::{
    ecs::{entity::Entity, event::Event, system::Resource},
    math::I64Vec2,
};

use crate::direction::Direction;

#[derive(Debug, PartialEq, Event)]
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

#[derive(Debug, PartialEq, Event)]
pub struct OrganismEvent(pub Entity, pub OrganismRequest);

#[derive(Debug, Resource)]
pub struct OrganismRequests(Vec<OrganismRequest>);

impl OrganismRequests {
    pub fn append(&mut self, requests: &mut Vec<OrganismRequest>) {
        self.0.append(requests);
    }
}
