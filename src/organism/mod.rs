use crate::{Actor, Direction, Drawable, OrganType, OrganismRequest, WorldSettings};
use anyhow::anyhow;
use bevy::{math::I64Vec2, render::color::Color};
use rand::Rng;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock, RwLockReadGuard},
};
use uuid::Uuid;

use super::Producer;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Organ {
    pub id: Uuid,
    pub r#type: OrganType,
    pub relative_location: I64Vec2,
}

/// The vectors in here are relatively positioned to the center of the organism
pub enum OrganEvent {
    MakeFoodAround(I64Vec2),
    EatFoodAround(I64Vec2),
    KillAround(I64Vec2),
}

impl Organ {
    pub fn new(r#type: OrganType, relative_location: I64Vec2) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            r#type,
            relative_location,
        }
    }

    pub fn new_rand(relative_location: I64Vec2) -> Organ {
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

    pub fn tick(&mut self, world_settings: &WorldSettings) -> Option<OrganEvent> {
        match self.r#type {
            OrganType::Producer(ref mut producer) => {
                producer.counter += 1;

                if producer.counter >= world_settings.producer_threshold {
                    producer.counter = 0;
                    return Some(OrganEvent::MakeFoodAround(self.relative_location));
                }

                None
            }
            OrganType::Mouth => Some(OrganEvent::EatFoodAround(self.relative_location)),

            OrganType::Killer => Some(OrganEvent::EatFoodAround(self.relative_location)),
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
    pub fn into_organism(self, location: I64Vec2) -> Organism {
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
    pub id: Uuid,
    r#type: OrganismType,
    organs: Vec<Arc<RwLock<Organ>>>,
    pub location: I64Vec2,
    facing: Direction,
    reproduce_at: u64,
    time_alive: u64,
    time_since_consumption: u64,
    belly: u64,
    offspring: u64,
    food_collected: u64,
    mutation_rate: f64,
}

impl Organism {
    pub fn actor(&self) -> Actor {
        Actor::Organism(self.id, self.location)
    }
    pub fn try_new(
        organs: Vec<Organ>,
        location: I64Vec2,
        mutation_rate: f64,
        facing: Direction,
        belly: u64,
    ) -> Result<Self, anyhow::Error> {
        let mut organism_type = OrganismType::None;
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
                _ => {}
            }
        }

        let reproduce_at = organs.len() * 3;

        Ok(Organism {
            id: Uuid::new_v4(),
            organs: organs
                .into_iter()
                .map(|o| Arc::new(RwLock::new(o)))
                .collect(),
            r#type: organism_type,
            reproduce_at: reproduce_at.try_into().unwrap(),
            location,
            facing,
            time_alive: 0,
            time_since_consumption: 0,
            offspring: 0,
            belly,
            food_collected: 0,
            mutation_rate,
        })
    }

    pub fn simple_producer(location: I64Vec2) -> Organism {
        let organs = vec![
            Organ::new(OrganType::Producer(Producer::new()), (-1, 1).into()),
            Organ::new(OrganType::Mouth, (0, 0).into()),
            Organ::new(OrganType::Producer(Producer::new()), (1, -1).into()),
        ];

        Organism::try_new(organs, location, 50., Direction::Right, 4).unwrap()
    }

    pub fn simple_mover(location: I64Vec2) -> Organism {
        let organs = vec![
            Organ::new(OrganType::Mouth, (0, 0).into()),
            Organ::new(OrganType::Mover, (1, -1).into()),
        ];

        Organism::try_new(organs, location, 50., Direction::Right, 4).unwrap()
    }

    pub fn turn_to(&mut self, face: Direction) {
        let turn_amount = self.facing.turn(face);
        self.facing = face;
        for organ in self.organs.iter() {
            let mut organ = organ.write().unwrap();
            let loc = &mut organ.relative_location;
            let og = *loc;
            match turn_amount {
                -1 => {
                    //counter-clockwise
                    loc.x = -og.y;
                    loc.y = og.x;
                }
                1 => {
                    //clockwise
                    loc.x = og.y;
                    loc.y = -loc.x;
                }
                2 => {
                    //opposite
                    loc.x = -og.x;
                    loc.y = -og.y;
                }
                0 => {
                    //none
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn randomize_direction(&mut self) {
        let original_direction = self.facing;
        self.facing.randomize();
        let new_direction = self.facing;

        let turn_amount = original_direction.turn(new_direction);
        for organ in self.organs.iter() {
            let mut organ = organ.write().unwrap();
            let loc = &mut organ.relative_location;
            let og = *loc;
            match turn_amount {
                -1 => {
                    //counter-clockwise
                    loc.x = -og.y;
                    loc.y = og.x;
                }
                1 => {
                    //clockwise
                    loc.x = og.y;
                    loc.y = -loc.x;
                }
                2 => {
                    //opposite
                    loc.x = -og.x;
                    loc.y = -og.y;
                }
                0 => {
                    //none
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn reverse_direction(&mut self) {
        self.facing.reverse();

        match self.facing {
            Direction::Down | Direction::Up => {
                //reverse the locations across the y axis
                for organ in self.organs.iter() {
                    let mut organ = organ.write().unwrap();
                    let loc = &mut organ.relative_location;

                    loc.y = -loc.y
                }
            }
            Direction::Left | Direction::Right => {
                //reverse the locations across the x axis
                for organ in self.organs.iter() {
                    let mut organ = organ.write().unwrap();
                    let loc = &mut organ.relative_location;

                    loc.x = -loc.x
                }
            }
        }
    }

    pub fn arc_organs(&self) -> impl Iterator<Item = (I64Vec2, &Arc<RwLock<Organ>>)> {
        self.organs.iter().map(|organ| {
            let organ_inner = organ.read().unwrap();
            (self.location + organ_inner.relative_location, organ)
        })
    }

    pub fn organs(&self) -> impl Iterator<Item = (I64Vec2, RwLockReadGuard<'_, Organ>)> + '_ {
        self.organs.iter().map(|organ| {
            let organ_inner = organ.read().unwrap();
            (self.location + organ_inner.relative_location, organ_inner)
        })
    }
    pub fn occupied_locations(&self) -> impl Iterator<Item = I64Vec2> + '_ {
        return self
            .organs
            .iter()
            .map(|organ| self.location + organ.read().unwrap().relative_location);
    }

    pub fn facing(&self) -> Direction {
        self.facing
    }

    pub fn collide(&mut self) {
        //change direction
        self.facing.randomize();
    }
    pub fn tick(&mut self, world_settings: &WorldSettings) -> Vec<OrganismRequest> {
        self.time_alive += 1;
        self.time_since_consumption += 1;

        if self.belly == 0 || self.time_alive == 200 {
            return vec![OrganismRequest::Starve];
        }

        if self.time_alive % world_settings.hunger_tick == 0 {
            self.belly -= 1;
            if self.r#type == OrganismType::Mover && self.time_since_consumption > 20 {
                self.facing.randomize();
            }
        }

        let mut requests = Vec::new();
        if self.belly >= self.reproduce_at * self.offspring {
            requests.push(OrganismRequest::Reproduce);
        }

        let mut eye_locations: Option<Vec<(I64Vec2, Direction)>> = None;
        for organ in self.organs.iter_mut() {
            let mut organ = organ.write().unwrap();
            if let OrganType::Eye(direction) = organ.r#type {
                if self.r#type == OrganismType::Mover {
                    match eye_locations {
                        Some(ref mut v) => {
                            v.push((self.location + organ.relative_location, direction))
                        }
                        None => {
                            eye_locations =
                                Some(vec![(self.location + organ.relative_location, direction)])
                        }
                    }
                }

                continue;
            }

            let Some(event) = organ.tick(world_settings) else {
                continue;
            };
            match event {
                OrganEvent::MakeFoodAround(relative_location) => {
                    if self.r#type == OrganismType::Mover {
                        continue;
                    }

                    requests.push(OrganismRequest::ProduceFoodAround(
                        relative_location + self.location,
                    ))
                }
                OrganEvent::EatFoodAround(relative_location) => {
                    requests.push(OrganismRequest::EatFoodAround(
                        self.location + relative_location,
                    ));
                }
                //this isn't right. Kill lgoic should not be done here.
                OrganEvent::KillAround(relative_location) => {
                    requests.push(OrganismRequest::KillAround(
                        self.location + relative_location,
                    ));
                }
            }
        }

        if self.r#type == OrganismType::Mover {
            if let Some(eye_locations) = eye_locations {
                requests.push(OrganismRequest::IntelligentMove(eye_locations))
            } else {
                requests.push(OrganismRequest::MoveBy(self.facing.delta()));
            }
        }
        requests
    }

    pub fn reproduce(&mut self) -> Result<NewSpawn, anyhow::Error> {
        let mut rng = rand::thread_rng();
        self.belly /= 2;

        let mut new_organs = self
            .organs()
            .map(|(_l, organ)| organ.clone())
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
                        return Err(anyhow!("the new spawn has no organs :("));
                    }
                    let index = rng.gen_range(0..new_organs.len());
                    new_organs.swap_remove(index);
                }

                MutationAction::New => {
                    let occupied_locations = new_organs
                        .iter()
                        .map(|o| o.relative_location)
                        .collect::<Vec<_>>();

                    let attach_to = if occupied_locations.is_empty() {
                        I64Vec2::new(0, 0)
                    } else {
                        //pick a random location in the list
                        *occupied_locations
                            .get(rng.gen_range(0..occupied_locations.len()))
                            .unwrap()
                    };

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
                            return Err(anyhow!(
                                "This spawn couldn't add an organ after 11 randomized attempts!"
                            ));
                        }
                        if occupied_locations.contains(&(attach_to + I64Vec2::new(x, y))) {
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
                            new_organs.push(Organ::new_rand(attach_to + I64Vec2::new(x, y)));
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
        self.offspring += 1;
        Ok(NewSpawn::new(
            new_organs,
            new_organism_mutability,
            self.belly,
            organism_direction,
        ))
    }

    pub fn get_color_for_cell(&self, location: &I64Vec2) -> Result<Color, anyhow::Error> {
        let relative_location = *location - self.location;
        for (_, organ) in self.organs() {
            if organ.relative_location == relative_location {
                return Ok(organ.color());
            }
        }
        Err(anyhow!("Organ not found!"))
    }

    pub fn move_by(&mut self, move_by: I64Vec2) {
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
