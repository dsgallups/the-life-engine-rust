use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

use crate::{Drawable, Organism};
use bevy::{
    ecs::system::Resource,
    math::{I64Vec3, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::Rng;
mod request;
use anyhow::anyhow;
pub use request::*;

mod map;
pub use map::*;
use uuid::Uuid;
//mod threading;
//use threading::*;

///holds the map and organisms
#[derive(Resource)]
#[allow(dead_code)]
pub struct LEWorld {
    settings: WorldSettings,
    map: RwLock<WorldMap>,
    organisms: Vec<Arc<RwLock<Organism>>>,
    lifetime: u64,
    graveyard: RwLock<Vec<Organism>>,
    n_threads: u64,
}

impl Default for LEWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl LEWorld {
    pub fn new() -> LEWorld {
        let thread_count = thread::available_parallelism().unwrap().get() as u64;
        LEWorld {
            settings: WorldSettings::default(),
            map: RwLock::new(WorldMap::new()),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: RwLock::new(Vec::new()),
            n_threads: thread_count,
        }
    }

    pub fn new_walled(length: u64) -> LEWorld {
        let thread_count = thread::available_parallelism().unwrap().get() as u64;
        LEWorld {
            settings: WorldSettings::default(),
            map: RwLock::new(WorldMap::new_walled(length)),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: RwLock::new(Vec::new()),
            n_threads: thread_count,
        }
    }

    pub fn add_simple_producer(&mut self, location: I64Vec3) {
        self.add_organism(Organism::simple_producer(location));
    }
    pub fn add_simple_mover(&mut self, location: I64Vec3) {
        self.add_organism(Organism::simple_mover(location));
    }
    pub fn add_organism(&mut self, organism: Organism) {
        let organism = Arc::new(RwLock::new(organism));

        self.insert_organism_into_map(&organism);

        self.organisms.push(organism);
    }

    pub fn insert_organism_into_map(&mut self, organism: &Arc<RwLock<Organism>>) {
        let mut map = self.map.write().unwrap();
        if let Err(e) = map.insert_organism(organism) {
            println!("{}", e);
        }
    }

    pub fn tick(&mut self) -> Result<(), anyhow::Error> {
        {
            let graveyard = self.graveyard.read().unwrap();
            println!(
                "tick {} - organism count: alive - {}, dead - {}",
                self.lifetime,
                self.organisms.len(),
                graveyard.len()
            );
        }

        self.lifetime += 1;
        if self.organisms.is_empty() {
            return Err(anyhow!("everyone died!!!"));
        }

        let requests = self
            .organisms
            .iter_mut()
            .map(|arc_organism| {
                let mut organism_lock = arc_organism.write().unwrap();
                let requests = organism_lock.tick(&self.settings);
                (Arc::clone(&arc_organism), requests)
            })
            .collect::<Vec<_>>();

        let (dead_list, mut new_spawn) = {
            let mut map = self.map.write().unwrap();
            requests.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut dead_list, mut new_spawn), (organism, requests)| {
                    //tosdowjefwio
                    //werwf
                    for request in requests {
                        match request {
                            OrganismRequest::ProduceFoodAround(location) => {
                                if let Err(e) = map
                                    .produce_food_around(location, self.settings.food_spawn_radius)
                                {
                                    println!("Couldn't make food: {}", e);
                                    continue;
                                }
                            }
                            OrganismRequest::MoveBy(location) => {
                                if let Err(e) = map.move_organism(&organism, location) {
                                    println!("Couldn't move organism: {}", e);
                                    //do someething
                                    continue;
                                }
                            }
                            OrganismRequest::EatFoodAround(location) => {
                                match map.feed_organism(&organism, location) {
                                    Ok(()) => {}
                                    Err(e) => println!("Error feeding organism: {}", e),
                                }
                            }

                            OrganismRequest::KillAround(location) => {
                                let mut killed = map.kill_around(&organism, location);
                                dead_list.append(&mut killed);
                            }
                            OrganismRequest::Starve => match map.kill_organism(&organism) {
                                Ok(id) => dead_list.push(id),
                                Err(e) => {
                                    println!("Error killing organism! {}", e);
                                }
                            },
                            OrganismRequest::Reproduce => {
                                match map.deliver_child(&organism, self.settings.spawn_radius) {
                                    Ok(child) => {
                                        new_spawn.push(child);
                                    }
                                    Err(e) => {
                                        println!("Error reproducing: {}", e);
                                    }
                                }
                            }
                        }
                    }

                    (dead_list, new_spawn)
                },
            )
        };

        if !dead_list.is_empty() {
            let (mut dead_organisms, alive_organisms) = self.organisms.clone().into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut dead_organisms, mut alive_organisms), organism| {
                    let dead = {
                        let org_lock = organism.read().unwrap();
                        dead_list.contains(&org_lock.id)
                    };

                    if dead {
                        //nothing should be holding onto this
                        let count = Arc::strong_count(&organism);
                        let Some(organism) = Arc::into_inner(organism) else {
                            println!("organism is still referenced! count is: {}", count);
                            return (dead_organisms, alive_organisms);
                        };

                        let organism = organism.into_inner().unwrap();
                        dead_organisms.push(organism);
                    } else {
                        alive_organisms.push(organism);
                    }

                    (dead_organisms, alive_organisms)
                },
            );
            self.organisms = alive_organisms;

            self.graveyard.write().unwrap().append(&mut dead_organisms);
        }

        self.organisms.append(&mut new_spawn);

        Ok(())
    }

    pub fn draw(&self) -> Vec<SpriteBundle> {
        let map = self.map.read().unwrap();

        let mut sprites: Vec<SpriteBundle> = Vec::with_capacity(self.organisms.len());

        for (location, square) in map.iter() {
            let color = square.color();
            sprites.push(SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform::from_translation(Vec3::new(
                    location.x as f32,
                    location.y as f32,
                    0.,
                )),
                ..default()
            });
        }
        sprites
    }
}

pub struct WorldSettings {
    pub food_spawn_radius: i64,
    pub producer_threshold: u8,
    //every nth tick of an organism being alive, decrease its food consumed by 1
    pub hunger_tick: u64,
    pub spawn_radius: u64,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            food_spawn_radius: 1,
            hunger_tick: 6,
            producer_threshold: 2,
            spawn_radius: 15,
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    pub fn delta(&self) -> I64Vec3 {
        match self {
            Direction::Up => I64Vec3::new(1, 0, 0),
            Direction::Down => I64Vec3::new(-1, 0, 0),
            Direction::Left => I64Vec3::new(0, -1, 0),
            Direction::Right => I64Vec3::new(0, 1, 0),
        }
    }
    pub fn reverse(&mut self) {
        match self {
            Direction::Up => *self = Direction::Down,
            Direction::Down => *self = Direction::Up,
            Direction::Left => *self = Direction::Left,
            Direction::Right => *self = Direction::Right,
        }
    }
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=3) {
            0 => *self = Direction::Down,
            1 => *self = Direction::Up,
            2 => *self = Direction::Left,
            3 => *self = Direction::Right,
            _ => unreachable!(),
        }
    }
}

#[test]
fn create_world() {
    let mut world = LEWorld::new();

    world.add_simple_producer((0, 0, 0).into());
}

#[test]
fn create_world_panic() {
    let mut world = LEWorld::new();
    world.add_simple_producer((0, 0, 0).into());
    world.add_simple_producer((0, 0, 0).into());
}
