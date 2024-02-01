use crate::map::WorldLocation;
mod request;
use super::direction::Direction;
use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity, event::Event},
    math::Vec3,
    sprite::SpriteBundle,
    transform::components::Transform,
};
use rand::Rng;
pub use request::*;
use std::fmt::Debug;
mod organ;
pub use organ::*;

#[derive(Default, Debug, Clone, PartialEq, Component)]
pub enum OrganismType {
    Mover,
    #[default]
    Producer,
    None,
}

#[derive(Event)]
pub struct Reproduce(pub Entity);

/*#[derive(Debug)]
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
*/

#[derive(Default, Clone, Bundle)]
pub struct OrganismBundle {
    pub sprite: SpriteBundle,
    pub location: WorldLocation,
    pub organism_type: OrganismType,
    pub organism_info: OrganismInfo,
}

impl OrganismBundle {
    pub fn new(
        organism_type: OrganismType,
        location: impl Into<WorldLocation>,
        organism_info: OrganismInfo,
    ) -> Self {
        let location: WorldLocation = location.into();
        Self {
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    location.x() as f32,
                    location.y() as f32,
                    0.,
                )),
                ..Default::default()
            },
            location,
            organism_type,
            organism_info,
        }
    }
}

#[derive(Default, Clone, Component)]
pub struct OrganismInfo {
    pub time_alive: u64,
    pub time_since_consumption: u64,
    pub belly: u64,
    pub mutation_rate: f64,
    pub food_collected: u64,
    pub facing: Direction,
}

impl OrganismInfo {
    pub fn new(initial_food: u64) -> Self {
        Self {
            belly: initial_food,
            ..Default::default()
        }
    }

    pub fn gen_child_stats(&mut self) -> Self {
        let mut rng = rand::thread_rng();

        self.belly /= 2;

        let mut new_stats = self.clone();

        if rng.gen::<bool>() {
            new_stats.mutation_rate += 1.
        } else {
            new_stats.mutation_rate -= 1.
        };

        new_stats.facing = match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!(),
        };

        new_stats
    }
}
/*
#[derive(Default, Debug, Clone, Component)]
#[allow(dead_code)]
pub struct Organism {
    pub id: Uuid,
    r#type: OrganismType,
    organs: Vec<Organ>,
    pub facing: Direction,
    reproduce_at: u64,
    time_alive: u64,
    time_since_consumption: u64,
    belly: u64,
    offspring: u64,
    food_collected: u64,
    mutation_rate: f64,
}

impl Organism {
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
            organs,
            r#type: organism_type,
            reproduce_at: reproduce_at.try_into().unwrap(),
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

    //only the map sould call this and update appropriately
    pub fn turn_to(&mut self, face: Direction) {
        let turn_amount = self.facing.turn(face);
        self.facing = face;
        for organ in self.organs.iter_mut() {
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
        for organ in self.organs.iter_mut() {
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
                for organ in self.organs.iter_mut() {
                    let loc = &mut organ.relative_location;

                    loc.y = -loc.y
                }
            }
            Direction::Left | Direction::Right => {
                //reverse the locations across the x axis
                for organ in self.organs.iter_mut() {
                    let loc = &mut organ.relative_location;

                    loc.x = -loc.x
                }
            }
        }
    }

    pub fn organs(&self) -> impl Iterator<Item = &Organ> {
        self.organs.iter()
    }

    pub fn facing(&self) -> Direction {
        self.facing
    }

    pub fn reproduce(&mut self) -> Result<NewSpawn, anyhow::Error> {
        let mut rng = rand::thread_rng();
        self.belly /= 2;

        let mut new_organs = self.organs().map(|organ| organ.clone()).collect::<Vec<_>>();

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

    pub fn feed(&mut self, amount: u64) {
        self.belly += amount;
    }
}


*/

pub enum MutationAction {
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

    pub fn rand_list(mutation_rate: f64) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut list = Vec::new();
        loop {
            let rng_val = rng.gen_range(0..=100);
            if rng_val as f64 <= mutation_rate {
                list.push(MutationAction::rand());
            } else {
                break;
            }
        }
        list
    }
}
