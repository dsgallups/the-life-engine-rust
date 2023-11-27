use bevy::math::I64Vec3;

/// The organism will provide the world for the context it needs in order for it to make a request
pub struct OrganismContextRequest {
    pub nearest_food: bool,
}

/// The world will provide the organism with a response to its context request
pub struct WorldContextResponse {
    pub nearest_food: Option<I64Vec3>,
}

/// From the context provided by the world, the organism will provide the world with a request to update the world
pub struct OrganismUpdateRequest {
    place_food: Vec<I64Vec3>,
}

/// The world will provide the organism with a response to its update request
pub struct WorldUpdateResponse {}
