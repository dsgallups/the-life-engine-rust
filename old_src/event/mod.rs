use bevy::math::I64Vec2;
use uuid::Uuid;

#[derive(Debug)]
pub enum EventType {
    FailKilled(String),
    Killed,
    FailStarved(String),
    Starved,
    MovedToGraveyard,
    NotMovedToGraveyard,
    FailAte(String),
    Ate,
    FailProduced(String),
    Moved,
    Exists,
    FailMoved(String),
    Produced,
    Reproduced,
    FailReproduced(String),
    DeadList,
}

#[derive(Debug)]
pub enum On {
    Actor(Actor),
    Actors(Vec<(Uuid, bool)>),
    Food(I64Vec2, u64),
    Around(I64Vec2),
    To(I64Vec2),
    None,
}

#[derive(Debug)]
pub enum Actor {
    Map,
    Organism(Uuid, I64Vec2),
}

#[derive(Debug)]
pub struct Event {
    pub tick: u64,
    pub actioner: Actor,
    pub action: EventType,
    pub on: On,
}
impl Event {
    pub fn new(tick: u64, actioner: Actor, action: EventType, on: On) -> Self {
        Self {
            tick,
            actioner,
            action,
            on,
        }
    }
}
