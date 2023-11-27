use bevy::{math::I64Vec3, utils::Uuid};

use super::Organ;

/// The organism will provide the world for the context it needs in order for it to make a request
pub struct OrganismContextRequest {}

/// The world will provide the organism with a response to its context request
pub struct WorldContextResponse {}

/// From the context provided by the world, the organism will provide the world with a request to update the world
#[derive(Default)]
pub struct OrganismUpdateRequest<'a> {
    pub gen_food: Vec<&'a Organ>,
}

impl<'a> OrganismUpdateRequest<'a> {
    pub fn add_gen_food(&mut self, organ: &'a Organ) {
        self.gen_food.push(organ);
    }
}

/// The world will provide the organism with a response to its update request
#[derive(Default)]
pub struct WorldUpdateResponse<'a> {
    pub gen_food: Vec<&'a Organ>,
}
