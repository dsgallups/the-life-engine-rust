use crate::{Direction, Drawable, OrganType, WorldMap, WorldRequest, WorldSettings};
use anyhow::anyhow;
use bevy::{math::I64Vec3, render::color::Color};
use rand::Rng;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
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

    pub fn new_rand(relative_location: I64Vec3) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            r#type: OrganType::new_rand(),
            relative_location,
        }
    }

    pub fn mutate(&mut self) {
        self.r#type = OrganType::new_rand();
    }

    pub fn organ_type(&self) -> &OrganType {
        &self.r#type
    }
    pub fn color(&self) -> Color {
        self.r#type.color()
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

#[derive(Debug)]
pub struct NewSpawn {
    pub organs: Vec<Organ>,
    mutation_rate: f64,
    belly: u64,
    facing: Direction,
}

impl NewSpawn {
    pub fn new(organs: Vec<Organ>, mutation_rate: f64, belly: u64, facing: Direction) -> Self {
        Self {
            organs,
            mutation_rate,
            belly,
            facing,
        }
    }
    pub fn into_organism(self, location: I64Vec3) -> Organism {
        Organism::try_new(
            self.organs,
            location,
            self.mutation_rate,
            self.facing,
            self.belly,
        )
        .unwrap()
    }
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Organism {
    id: Uuid,
    r#type: OrganismType,
    organs: Vec<Arc<Mutex<Organ>>>,
    pub location: I64Vec3,
    facing: Direction,
    has_eye: bool,
    reproduce_at: u64,
    time_alive: u64,
    belly: u64,
    food_collected: u64,
    mutation_rate: f64,
}

impl Organism {
    pub fn try_new(
        organs: Vec<Organ>,
        location: I64Vec3,
        mutation_rate: f64,
        facing: Direction,
        belly: u64,
    ) -> Result<Self, anyhow::Error> {
        let mut organism_type = OrganismType::None;
        let mut has_eye = false;
        for organ in organs.iter() {
            match organ.organ_type() {
                OrganType::Producer(_) => {
                    if organism_type == OrganismType::None {
                        organism_type = OrganismType::Producer;
                    }
                }
                OrganType::Mover => {
                    organism_type = OrganismType::Mover;
                    break;
                }
                OrganType::Eye => has_eye = true,
                _ => {}
            }
        }

        let reproduce_at = organs.len() * 3;

        Ok(Organism {
            id: Uuid::new_v4(),
            organs: organs
                .into_iter()
                .map(|o| Arc::new(Mutex::new(o)))
                .collect(),
            r#type: organism_type,
            has_eye,
            reproduce_at: reproduce_at.try_into().unwrap(),
            location,
            facing,
            time_alive: 0,
            belly,
            food_collected: 0,
            mutation_rate,
        })
    }

    pub fn simple_producer(location: I64Vec3) -> Organism {
        let organs = vec![
            Organ::new(OrganType::Producer(Producer::new()), (-1, 1, 0).into()),
            Organ::new(OrganType::Mouth, (0, 0, 0).into()),
            Organ::new(OrganType::Producer(Producer::new()), (1, -1, 0).into()),
        ];

        Organism::try_new(organs, location, 50., Direction::Right, 4).unwrap()
    }

    pub fn simple_mover(location: I64Vec3) -> Organism {
        let organs = vec![
            Organ::new(OrganType::Mouth, (0, 0, 0).into()),
            Organ::new(OrganType::Mover, (1, -1, 0).into()),
        ];

        Organism::try_new(organs, location, 50., Direction::Right, 4).unwrap()
    }

    pub fn reverse_direction(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..10) == 9 {
            self.facing.randomize();
        } else {
            self.facing.reverse();
        }
    }

    pub fn organs(&self) -> impl Iterator<Item = (I64Vec3, &'_ Arc<Mutex<Organ>>)> + '_ {
        return self.organs.iter().map(|organ| {
            let organ_inner = organ.lock().unwrap();
            (self.location + organ_inner.relative_location, organ)
        });
    }
    pub fn occupied_locations(&self) -> impl Iterator<Item = I64Vec3> + '_ {
        return self
            .organs
            .iter()
            .map(|organ| self.location + organ.lock().unwrap().relative_location);
    }

    pub fn tick(&mut self, map: &WorldMap, world_settings: &WorldSettings) -> Vec<WorldRequest> {
        self.time_alive += 1;

        if self.time_alive % world_settings.hunger_tick == 0 {
            self.belly -= 1;
        }

        if self.belly == 0 || self.time_alive == 200 {
            return vec![WorldRequest::Starve];
        }

        let mut requests = Vec::new();
        if self.belly >= self.reproduce_at {
            requests.push(WorldRequest::Reproduce);
        }

        for organ in self.organs.iter_mut() {
            let mut organ = organ.lock().unwrap();
            let Some(event) = organ.tick(map, self.location, world_settings) else {
                continue;
            };
            match event {
                OrganismEvent::MakeFood(location) => {
                    if self.r#type == OrganismType::Mover {
                        continue;
                    }

                    requests.push(WorldRequest::Food(location + self.location))
                }
                OrganismEvent::EatFood(locations) => {
                    for location in locations {
                        requests.push(WorldRequest::EatFood(location))
                    }
                }
            }
        }
        if self.r#type == OrganismType::Mover {
            requests.push(WorldRequest::MoveBy(self.facing.delta()));
        }
        requests
    }

    pub fn reproduce(&mut self) -> Option<NewSpawn> {
        let mut rng = rand::thread_rng();
        self.belly /= 2;

        let mut new_organs = self
            .organs()
            .map(|(_l, organ)| {
                let organ = organ.lock().unwrap();
                organ.clone()
            })
            .collect::<Vec<_>>();

        let new_organism_mutability = if rng.gen::<bool>() {
            self.mutation_rate + 1.
        } else {
            self.mutation_rate - 1.
        };

        let mut mutation_list: Vec<MutationAction> = Vec::new();
        loop {
            let rng_val = rng.gen_range(0..=100);
            if rng_val as f64 <= new_organism_mutability {
                mutation_list.push(MutationAction::rand());
            } else {
                break;
            }
        }

        let organism_direction = match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!(),
        };

        for mutation in mutation_list {
            match mutation {
                MutationAction::Delete => {
                    if new_organs.is_empty() {
                        return None;
                    }
                    let index = rng.gen_range(0..new_organs.len());
                    new_organs.swap_remove(index);
                }

                MutationAction::New => {
                    let occupied_locations = new_organs
                        .iter()
                        .map(|o| o.relative_location)
                        .collect::<Vec<_>>();

                    if occupied_locations.is_empty() {
                        return None;
                    }

                    //pick a random location in the list
                    let attach_to = occupied_locations
                        .get(rng.gen_range(0..occupied_locations.len()))
                        .unwrap();

                    //pick a random place to start
                    let mut x = rng.gen_range(-1..=1);
                    let mut y = rng.gen_range(-1..=1);
                    if x == 0 && y == 0 {
                        if rng.gen::<bool>() {
                            x = if rng.gen::<bool>() { 1 } else { -1 };
                        } else {
                            y = if rng.gen::<bool>() { 1 } else { -1 };
                        }
                    }

                    let mut count = 0;
                    loop {
                        if count > 11 {
                            return None;
                        }
                        if occupied_locations.contains(&(*attach_to + I64Vec3::new(x, y, 0))) {
                            if x == 1 {
                                if y == -1 {
                                    y = 0
                                } else if y == 0 {
                                    y = 1
                                } else if y == 1 {
                                    x = 0
                                }
                            } else if x == 0 {
                                if y == -1 {
                                    x = 1
                                } else if y == 1 {
                                    x = -1
                                }
                            } else if x == -1 {
                                if y == -1 {
                                    x = 0;
                                } else if y == 0 {
                                    y = -1;
                                } else if y == 1 {
                                    y = 0;
                                }
                            }
                            count += 1;
                        } else {
                            new_organs.push(Organ::new_rand(*attach_to + I64Vec3::new(x, y, 0)));
                            break;
                        }
                    }
                }
                MutationAction::MutateOrgan => {
                    let new_organs_len = new_organs.len();
                    let organ_to_mutate = new_organs
                        .get_mut(rng.gen_range(0..new_organs_len))
                        .unwrap();
                    organ_to_mutate.mutate();
                }
            }
        }
        Some(NewSpawn::new(
            new_organs,
            new_organism_mutability,
            self.belly,
            organism_direction,
        ))
    }

    pub fn get_color_for_cell(&self, location: &I64Vec3) -> Result<Color, anyhow::Error> {
        let relative_location = *location - self.location;
        for organ in self.organs.iter() {
            let organ = organ.lock().unwrap();
            if organ.relative_location == relative_location {
                return Ok(organ.color());
            }
        }
        Err(anyhow!("Organ not found!"))
    }

    pub fn move_by(&mut self, move_by: I64Vec3) {
        self.location += move_by;
    }

    pub fn feed(&mut self, amount: u64) {
        self.belly += amount;
    }
}

enum MutationAction {
    Delete,
    New,
    MutateOrgan,
}

impl MutationAction {
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..2) {
            0 => MutationAction::Delete,
            1 => MutationAction::New,
            2 => MutationAction::MutateOrgan,
            _ => panic!(),
        }
    }
}
