/// The organism will provide the world for the context it needs in order for it to make a request
pub struct OrganismContextRequest {}

/// The world will provide the organism with a response to its context request
pub struct WorldContextResponse {}

/// From the context provided by the world, the organism will provide the world with a request to update the world

#[derive(Debug)]
pub enum OrganismUpdateRequest {
    GenFood(Uuid, I64Vec3),
}

/// The world will provide the organism with a response to its update request
#[derive(Debug)]
pub enum WorldUpdateResponse {
    ClearCounter(Uuid),
}
