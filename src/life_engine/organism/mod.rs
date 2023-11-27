use crate::life_engine::OrganismCell;
use bevy::math::I64Vec3;
use bevy::math::U64Vec3;
use bevy::prelude::*;
use bevy::utils::Uuid;
use std::fmt::Debug;

use super::OrganismContextRequest;
use super::OrganismUpdateRequest;
use super::WorldContextResponse;
use super::WorldUpdateResponse;

#[derive(Clone, Debug, Default)]
pub struct Organ {
    cell: OrganismCell,
    relative_location: I64Vec3,
}

impl Organ {
    pub fn new(cell: OrganismCell, relative_location: I64Vec3) -> Organ {
        Organ {
            cell,
            relative_location,
        }
    }
    pub fn position(&self) -> &I64Vec3 {
        &self.relative_location
    }

    pub fn cell(&self) -> OrganismCell {
        self.cell.clone()
    }
}

#[derive(Default, Component)]
pub struct Organism {
    id: Uuid,
    organs: Vec<Organ>,
    location: U64Vec3,
    has_eye: bool,
    food_collected: u64,
}

impl Organism {
    pub fn new(organs: Vec<Organ>, location: U64Vec3) -> Organism {
        Organism {
            id: Uuid::new_v4(),
            organs,
            has_eye: false,
            location,
            food_collected: 0,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn origin(&self) -> &U64Vec3 {
        &self.location
    }

    pub fn organs(&self) -> &[Organ] {
        &self.organs
    }

    pub fn context_request(&self) -> OrganismContextRequest {
        OrganismContextRequest {
            nearest_food: self.has_eye,
        }
    }

    pub fn update_request(&self, context_response: WorldContextResponse) -> OrganismUpdateRequest {
        todo!();
    }

    pub fn tick(&mut self, context_response: WorldUpdateResponse) {
        todo!();
    }
}
