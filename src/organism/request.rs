use bevy::ecs::{entity::Entity, event::Event, system::Resource};

use crate::{direction::Direction, map::WorldLocation};

#[derive(Debug, PartialEq, Event)]
/// The vector provided in all of these enums are absolutely positioned
pub enum OrganismRequest {
    /// Asks to move up by this amount
    MoveBy(WorldLocation),
    IntelligentMove(Vec<(WorldLocation, Direction)>),
    ProduceFoodAround(WorldLocation),
    KillAround(WorldLocation),
    EatFoodAround(WorldLocation),
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
