use bevy::prelude::*;
use uuid::Uuid;

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(Component, Reflect)]
pub struct Edge {
    id: Uuid,
    sender: Entity,
    receiver: Entity,
}
pub const LINE_MESH_X: f32 = 1.;
pub const LINE_MESH_Y: f32 = 2.;

impl Edge {
    pub fn new(id: Uuid, sender: Entity, receiver: Entity) -> Self {
        Self {
            sender,
            id,
            receiver,
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn sender(&self) -> Entity {
        self.sender
    }
    pub fn receiver(&self) -> Entity {
        self.receiver
    }
}
