use bevy::math::I64Vec2;
use uuid::Uuid;

#[derive(Debug)]
pub enum EventType {
    FailKilled(String),
    Killed,
    FailStarved(String),
    Starved,
    MovedToGraveyard,
    FailAte(String),
    Ate,
    FailProduced(String),
    Moved,
    FailMoved(String),
    Produced,
}

#[derive(Debug)]
pub enum On {
    Actor(Actor),
    Food(I64Vec2),
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
