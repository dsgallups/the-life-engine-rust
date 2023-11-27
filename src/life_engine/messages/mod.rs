use bevy::math::I64Vec3;

/// The organism will provide the world for the context it needs in order for it to make a request
pub struct OrganismContextRequest {}

/// The world will provide the organism with a response to its context request
pub struct WorldContextResponse {}

/// From the context provided by the world, the organism will provide the world with a request to update the world
#[derive(Default)]
pub struct OrganismUpdateRequest {
    gen_food: Vec<I64Vec3>,
}

impl OrganismUpdateRequest {
    pub fn add_gen_food(&mut self, position: I64Vec3) {
        self.gen_food.push(position);
    }
}

/// The world will provide the organism with a response to its update request
pub struct WorldUpdateResponse {}
