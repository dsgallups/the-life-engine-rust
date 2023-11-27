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
    pub id: Uuid,
    pub cell: OrganismCell,
    pub relative_location: I64Vec3,
}

impl Organ {
    pub fn new(cell: OrganismCell, relative_location: I64Vec3) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            cell,
            relative_location,
        }
    }
}

#[derive(Default, Component)]
pub struct Organism {
    pub id: Uuid,
    pub organs: Vec<Organ>,
    pub location: U64Vec3,
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

    pub fn context_request(&self) -> OrganismContextRequest {
        OrganismContextRequest {}
    }

    pub fn update_request(
        &mut self,
        _context_response: WorldContextResponse,
    ) -> Vec<OrganismUpdateRequest> {
        let mut request = Vec::new();

        /*if !self.has_eye {
            request.add_organ(Organ::new(OrganismCell::Producer(0), (0, 0, 1).into()));
        }*/

        for organ in self.organs.iter_mut() {
            println!("organ = {:?}", organ);
            if let OrganismCell::Producer(ref mut producer) = organ.cell {
                producer.counter += 1;
                println!("producer = {:?}", producer);

                if producer.counter >= producer.threshold {
                    request.push(OrganismUpdateRequest::GenFood(
                        organ.id,
                        organ.relative_location,
                    ));
                    println!("threshold exceeded!");
                }
            }
        }

        request
    }

    pub fn tick(&mut self, update_response: Vec<WorldUpdateResponse>) {
        for response in update_response {
            match response {
                WorldUpdateResponse::ClearCounter(organ_id) => {
                    if let Some(organ) = self.organs.iter_mut().find(|organ| organ.id == organ_id) {
                        if let OrganismCell::Producer(ref mut producer) = organ.cell {
                            println!("clearing counter");
                            producer.counter = 0;
                        }
                    }
                }
            }
        }
    }
}
