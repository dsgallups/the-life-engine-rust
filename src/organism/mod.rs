use crate::{OrganType, WorldMap, WorldRequest, WorldSettings};
use anyhow::anyhow;
use bevy::math::I64Vec3;
use std::fmt::Debug;
use uuid::Uuid;

use super::Producer;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Organ {
    pub id: Uuid,
    pub r#type: OrganType,
    pub relative_location: I64Vec3,
}

pub enum OrganismEvent {
    MakeFood(I64Vec3),
    EatFood(Vec<I64Vec3>),
}

impl Organ {
    pub fn new(r#type: OrganType, relative_location: I64Vec3) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            r#type,
            relative_location,
        }
    }
    pub fn organ_type(&self) -> &OrganType {
        &self.r#type
    }

    pub fn tick(
        &mut self,
        map: &WorldMap,
        organism_location: I64Vec3,
        world_settings: &WorldSettings,
    ) -> Option<OrganismEvent> {
        match self.r#type {
            OrganType::Producer(ref mut producer) => {
                producer.counter += 1;

                if producer.counter >= world_settings.producer_threshold {
                    producer.counter = 0;
                    return Some(OrganismEvent::MakeFood(self.relative_location));
                }

                None
            }
            OrganType::Mouth => map
                .get_food_around(self.relative_location + organism_location)
                .map(OrganismEvent::EatFood),
            _ => None,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub enum OrganismType {
    Mover,
    #[default]
    Producer,
    None,
}

#[derive(Default, Debug, PartialEq)]
pub struct Organism {
    id: Uuid,
    r#type: OrganismType,
    organs: Vec<Organ>,
    pub location: I64Vec3,
    has_eye: bool,
    time_alive: u64,
    belly: u64,
    food_collected: u64,
}

impl Organism {
    pub fn try_new(organs: Vec<Organ>, location: I64Vec3) -> Result<Self, anyhow::Error> {
        let mut organism_type = OrganismType::None;
        let mut has_eye = false;
        for organ in organs.iter() {
            match organ.organ_type() {
                OrganType::Producer(_) => match organism_type {
                    OrganismType::None => organism_type = OrganismType::Producer,
                    OrganismType::Mover => {
                        return Err(anyhow!(
                            "Organism cannot have a producer cell and have a mover cell!"
                        ))
                    }
                    _ => {}
                },
                OrganType::Mover => match organism_type {
                    OrganismType::None => organism_type = OrganismType::Mover,
                    OrganismType::Producer => {
                        return Err(anyhow!(
                            "Organism cannot have a producer cell and have a mover cell!"
                        ))
                    }
                    _ => {}
                },
                OrganType::Eye => has_eye = true,
                _ => {}
            }
        }

        Ok(Organism {
            id: Uuid::new_v4(),
            organs,
            r#type: organism_type,
            has_eye,
            location,
            time_alive: 0,
            belly: 4,
            food_collected: 0,
        })
    }

    pub fn new_simple(location: I64Vec3) -> Organism {
        let organs = vec![
            Organ::new(OrganType::Producer(Producer::new()), (-1, 1, 0).into()),
            Organ::new(OrganType::Mouth, (0, 0, 0).into()),
            Organ::new(OrganType::Producer(Producer::new()), (1, -1, 0).into()),
        ];

        Organism::try_new(organs, location).unwrap()
    }

    pub fn occupied_locations(&self) -> impl Iterator<Item = I64Vec3> + '_ {
        return self
            .organs
            .iter()
            .map(|organ| self.location + organ.relative_location);
    }

    pub fn tick(&mut self, map: &WorldMap, world_settings: &WorldSettings) -> Vec<WorldRequest> {
        self.time_alive += 1;

        if self.time_alive % world_settings.hunger_tick == 0 {
            self.belly -= 1;
        }

        if self.belly == 0 {
            return vec![WorldRequest::Starve];
        }

        let mut requests = Vec::new();
        for organ in self.organs.iter_mut() {
            let Some(event) = organ.tick(map, self.location, world_settings) else {
                continue;
            };
            match event {
                OrganismEvent::MakeFood(location) => {
                    requests.push(WorldRequest::Food(location + self.location))
                }
                OrganismEvent::EatFood(locations) => {
                    for location in locations {
                        requests.push(WorldRequest::EatFood(location))
                    }
                }
            }
        }
        requests
    }

    pub fn move_by(&mut self, move_by: I64Vec3) {
        self.location += move_by;
    }

    pub fn feed(&mut self, amount: u64) {
        self.belly += amount;
    }
}
