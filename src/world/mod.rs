use std::{
    sync::{Arc, RwLock},
    thread,
};

use crate::{Cell, Drawable, Event, EventType, On, Organism};
use bevy::{
    ecs::system::Resource,
    math::{I64Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::{seq::SliceRandom, Rng};
mod neighbors;
mod request;
#[cfg(feature = "log")]
use crate::Actor;
use anyhow::anyhow;
pub use request::*;
#[cfg(feature = "log")]
use uuid::Uuid;

///holds the map and organisms
#[derive(Resource, Debug)]
#[allow(dead_code)]
pub struct LEWorld {
    settings: WorldSettings,
    organisms: Vec<Organism>,
    lifetime: u64,
    graveyard: Vec<Organism>,
    n_threads: u64,
    #[cfg(feature = "log")]
    events: Vec<Event>,
    pub paused: bool,
}

impl Default for LEWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl LEWorld {
    pub fn new() -> LEWorld {
        let thread_count = thread::available_parallelism().unwrap().get() as u64;
        let settings = Arc::new(WorldSettings::default());
        LEWorld {
            settings: WorldSettings::default(),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: Vec::new(),
            n_threads: thread_count,
            paused: false,
            #[cfg(feature = "log")]
            events: Vec::new(),
        }
    }

    pub fn new_walled(max_organisms: usize, length: u64) -> LEWorld {
        let settings = WorldSettings {
            max_organisms: Some(max_organisms),
            wall_length_half: Some((length / 2) as i64),
            ..Default::default()
        };

        let thread_count = thread::available_parallelism().unwrap().get() as u64;
        LEWorld {
            settings,
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: Vec::new(),
            n_threads: thread_count,
            paused: false,
            #[cfg(feature = "log")]
            events: Vec::new(),
        }
    }

    #[cfg(feature = "log")]
    pub fn push_evt(&mut self, actor: Actor, evt: EventType, on: On) {
        self.events.push(Event::new(self.lifetime, actor, evt, on))
    }

    pub fn add_simple_producer(&mut self, location: I64Vec2) {
        self.add_organism(Organism::simple_producer(location));
    }
    pub fn add_simple_mover(&mut self, location: I64Vec2) {
        self.add_organism(Organism::simple_mover(location));
    }
    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    pub fn simple_log(&mut self) {
        println!(
            "TICK {}: Alive - {}; Dead - {};",
            self.lifetime,
            self.organisms.len(),
            self.graveyard.len()
        );
    }

    pub fn reset(&mut self) {
        *self = LEWorld::new();

        self.add_simple_producer((0, 0).into());
    }

    pub fn decimate(&mut self) {
        let num_organisms_to_kill = self.organisms.len() / 2;
        let mut rng = rand::thread_rng();
        self.organisms.shuffle(&mut rng);
        todo!();
    }

    pub fn limit_organism_population(&mut self, population: Option<usize>) {
        let mut new_settings = self.settings.clone();
        new_settings.max_organisms = population;
        self.settings = new_settings;
    }

    pub fn pause(&mut self) {
        #[cfg(feature = "log")]
        println!("paused!");

        self.paused = true;
    }

    pub fn unpause(&mut self) {
        #[cfg(feature = "log")]
        println!("unpaused!");
        self.paused = false;
    }

    pub fn postmortem(&self) {
        println!("{:#?}", self.graveyard);

        #[cfg(feature = "log")]
        for organism in self.graveyard.iter() {
            self.log_id(organism.id);
        }
    }

    pub fn check_alive(&self) {
        println!("Vec: Live Organism Count - {}", self.organisms.len());
        let mut alive_organ_count = 0;
        for organism in self.organisms.iter() {
            let count = organism.organs().count();
            alive_organ_count += count;
        }
        println!("Vec: Live Organ Count - {}", alive_organ_count);
    }
    #[cfg(feature = "log")]
    pub fn log_id(&self, id: Uuid) {
        println!("====begin event search====");
        for event in self.events.iter() {
            if let Actor::Organism(o, _location) = event.actioner {
                if o == id {
                    println!("{:?}", event);
                    continue;
                }
            }
            if let On::Actor(Actor::Organism(o, _loc)) = event.on {
                if o == id {
                    println!("{:?}", event);
                }
            }
            if let On::Actors(uuids) = &event.on {
                for uuid in uuids {
                    if uuid.0 == id {
                        println!("{:?}", event);
                        break;
                    }
                }
            }
        }
        println!("====end of events====");
    }

    pub fn log_square(&self, position: I64Vec2) {
        todo!();
    }

    pub fn log(&self) {
        println!("World Information:");
        println!("Alive Organisms: {:?}", self.organisms);
    }

    pub fn tick(&mut self) -> Result<(), anyhow::Error> {
        if self.paused {
            return Ok(());
        }

        self.lifetime += 1;
        if self.organisms.is_empty() {
            return Err(anyhow!("everyone died!!!"));
        }

        let requests = self
            .organisms
            .iter_mut()
            .map(|organism| {
                let requests = organism.tick(&self.settings);
                requests
            })
            .collect::<Vec<_>>();

        Ok(())
    }

    pub fn remove_dead(&mut self, list: Vec<Arc<RwLock<Organism>>>) {
        if !list.is_empty() {
            let uuid_list = list
                .into_iter()
                .map(|o| o.read().unwrap().id)
                .collect::<Vec<_>>();

            let mut organism_list = Vec::new();
            for organism in self.organisms.iter() {
                organism_list.push(organism.id);
            }

            let (mut dead_organisms, alive_organisms, mut _events) =
                self.organisms.clone().into_iter().fold(
                    (Vec::new(), Vec::new(), Vec::<Event>::new()),
                    |(mut dead_organisms, mut alive_organisms, mut _events), organism| {
                        let dead = { uuid_list.contains(&organism.id) };

                        if dead {
                            #[cfg(feature = "log")]
                            _events.push(Event::new(
                                self.lifetime,
                                Actor::Map,
                                EventType::MovedToGraveyard,
                                On::Actor(organism.actor()),
                            ));
                            dead_organisms.push(organism);
                        } else {
                            alive_organisms.push(organism);
                        }

                        (dead_organisms, alive_organisms, _events)
                    },
                );
            self.organisms = alive_organisms;

            self.graveyard.append(&mut dead_organisms);

            #[cfg(feature = "log")]
            self.events.append(&mut _events);
        }
    }

    pub fn draw(&self) -> Vec<SpriteBundle> {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct WorldSettings {
    pub producer_probability: u8,
    //every nth tick of an organism being alive, decrease its food consumed by 1
    pub hunger_tick: u64,
    pub spawn_radius: u64,
    pub max_organisms: Option<usize>,
    pub wall_length_half: Option<i64>,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            hunger_tick: 30,
            producer_probability: 5,
            spawn_radius: 15,
            max_organisms: None,
            wall_length_half: None,
        }
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    pub fn delta(&self) -> I64Vec2 {
        match self {
            Direction::Up => I64Vec2::new(0, 1),
            Direction::Down => I64Vec2::new(0, -1),
            Direction::Left => I64Vec2::new(-1, 0),
            Direction::Right => I64Vec2::new(1, 0),
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
    pub fn to_reversed(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Left,
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
    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=3) {
            0 => Direction::Down,
            1 => Direction::Up,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }

    pub fn turn(&self, other: Direction) -> i8 {
        use Direction::*;
        match (self, other) {
            (Up, Left) | (Left, Down) | (Down, Right) | (Right, Up) => -1,
            (Up, Right) | (Right, Down) | (Down, Left) | (Left, Up) => 1,
            (Up, Down) | (Left, Right) | (Down, Up) | (Right, Left) => 2,
            (Up, Up) | (Down, Down) | (Left, Left) | (Right, Right) => 0,
        }
    }
}

#[test]
fn create_world() {
    let mut world = LEWorld::new();

    world.add_simple_producer((0, 0).into());
}

#[test]
fn create_world_panic() {
    let mut world = LEWorld::new();
    world.add_simple_producer((0, 0).into());
    world.add_simple_producer((0, 0).into());
}
