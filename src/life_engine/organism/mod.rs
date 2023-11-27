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
    id: Uuid,
    cell: OrganismCell,
    relative_location: I64Vec3,
}

impl Organ {
    pub fn new(cell: OrganismCell, relative_location: I64Vec3) -> Organ {
        Organ {
            id: Uuid::new_v4(),
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
        OrganismContextRequest {}
    }

    pub fn update_request(&self, _context_response: WorldContextResponse) -> OrganismUpdateRequest {
        let mut request = OrganismUpdateRequest::default();

        /*if !self.has_eye {
            request.add_organ(Organ::new(OrganismCell::Producer(0), (0, 0, 1).into()));
        }*/

        for organ in self.organs() {
            if let OrganismCell::Producer(ref producer) = organ.cell() {
                if producer.counter > producer.threshold {
                    request.add_gen_food(organ);
                }
            }
        }

        request
    }

    pub fn tick(
        &mut self,
        _context_response: &WorldUpdateResponse,
    ) -> Option<OrganismUpdateRequest> {
        for organ in self.organs.iter_mut() {
            if let OrganismCell::Producer(ref mut producer) = organ.cell() {
                producer.counter += 1;
            }
        }

        None
    }
}
