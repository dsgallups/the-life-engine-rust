use std::{
    sync::{Arc, RwLock},
    thread,
};

use crate::{Actor, Cell, Drawable, Event, EventType, On, Organism};
use bevy::{
    ecs::system::Resource,
    math::{I64Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::{seq::SliceRandom, Rng};
mod request;
use anyhow::anyhow;
pub use request::*;

mod map;
pub use map::*;
use rustc_hash::FxHashSet;
use uuid::Uuid;

///holds the map and organisms
#[derive(Resource, Debug)]
#[allow(dead_code)]
pub struct LEWorld {
    settings: WorldSettings,
    map: RwLock<WorldMap>,
    organisms: Vec<Arc<RwLock<Organism>>>,
    lifetime: u64,
    graveyard: Vec<Arc<RwLock<Organism>>>,
    n_threads: u64,
    max_organisms: Option<usize>,
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
        LEWorld {
            settings: WorldSettings::default(),
            map: RwLock::new(WorldMap::new()),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: Vec::new(),
            n_threads: thread_count,
            paused: false,
            max_organisms: None,
            events: Vec::new(),
        }
    }

    pub fn new_walled(length: u64) -> LEWorld {
        let thread_count = thread::available_parallelism().unwrap().get() as u64;
        LEWorld {
            settings: WorldSettings::default(),
            map: RwLock::new(WorldMap::new_walled(length)),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: Vec::new(),
            n_threads: thread_count,
            paused: false,
            events: Vec::new(),
            max_organisms: None,
        }
    }

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

    pub fn reset(&mut self) {
        *self = LEWorld::new();

        self.add_simple_producer((0, 0).into());
    }

    pub fn decimate(&mut self) {
        let num_organisms_to_kill = self.organisms.len() / 2;
        let mut rng = rand::thread_rng();
        self.organisms.shuffle(&mut rng);

        let mut map = self.map.write().unwrap();

        let mut dead_list = Vec::new();

        for (index, organism) in self.organisms.iter().enumerate() {
            match map.kill_organism(organism) {
                Ok(id) => dead_list.push(id),
                Err(e) => {
                    println!("failure killing organism: {}", e)
                }
            }
            if index > num_organisms_to_kill {
                break;
            }
        }
    }

    pub fn limit_organism_population(&mut self, population: Option<usize>) {
        self.max_organisms = population;
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

    pub fn check_alive(&mut self) {
        let map = self.map.read().unwrap();

        let mut live_organisms_in_map = FxHashSet::default();
        let mut live_organs_in_map_count = 0;
        for (_x, y) in map.iter() {
            if let Cell::Organism(o, _) = y {
                let o = o.read().unwrap();
                live_organisms_in_map.insert(o.id);
                live_organs_in_map_count += 1;
            }
        }

        println!("Vec: Live Organism Count - {}", self.organisms.len());
        let mut alive_organ_count = 0;
        for organism in self.organisms.iter() {
            let o = organism.read().unwrap();
            let count = o.organs().count();
            alive_organ_count += count;
        }
        println!("Vec: Live Organ Count - {}", alive_organ_count);

        println!("Map; Live Organism count - {}", live_organisms_in_map.len());
        println!("Map: Live Organ count - {}", live_organs_in_map_count);

        if self.organisms.len() > live_organisms_in_map.len() {
            for organism in self.organisms.iter() {
                let o = organism.read().unwrap();
                println!("ONE IS MISSING!");
                if !live_organisms_in_map.contains(&o.id) {
                    self.log_id(o.id);
                }
            }
        }
    }

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
        }
        println!("====end of events====");
    }

    pub fn log_square(&self, position: I64Vec2) {
        let map = self.map.read().unwrap();

        let square = map.get(&position);

        println!("Square clicked: \n{:#?}", square);

        let mut id = None;

        if let Some(Cell::Organism(organism, _)) = square {
            let questioned_organism = organism.read().unwrap();
            id = Some(questioned_organism.id);
            let mut found = false;
            for o in self.organisms.iter() {
                let o = o.read().unwrap();

                if o.id == questioned_organism.id {
                    println!("this oragnism is in our alive");
                    found = true;
                    break;
                }
            }

            if !found {
                println!("this organism is not found alive");
            }
        }

        let Some(id) = id else {
            return;
        };

        self.log_id(id);
    }

    pub fn log(&self) {
        println!("World Information:");
        println!("Alive Organisms: {:?}", self.organisms);
    }

    pub fn tick(&mut self) -> Result<(), anyhow::Error> {
        if self.paused {
            return Ok(());
        }

        #[cfg(feature = "log")]
        {
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
                (Arc::clone(arc_organism), requests)
            })
            .collect::<Vec<_>>();

        let (dead_list, mut new_spawn, critical_errors, mut events) = {
            let mut map = self.map.write().unwrap();
            requests.into_iter().fold(
                (
                    Vec::new(),
                    Vec::new(),
                    Vec::<anyhow::Error>::new(),
                    Vec::new(),
                ),
                |(mut dead_list, mut new_spawn, mut errors, mut events), (organism, requests)| {
                    //tosdowjefwio
                    //werwf
                    for request in requests {
                        match request {
                            OrganismRequest::ProduceFoodAround(location) => {
                                match map
                                    .produce_food_around(location, self.settings.food_spawn_radius)
                                {
                                    Ok(()) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::Produced,
                                            On::Around(location),
                                        ));
                                    }
                                    Err(e) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::FailProduced(e.to_string()),
                                            On::Around(location),
                                        ));
                                    }
                                }
                            }
                            OrganismRequest::MoveBy(location) => {
                                match map.move_organism(&organism, location) {
                                    Ok(()) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::Moved,
                                            On::To(location),
                                        ))
                                    }
                                    Err(e) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::FailMoved(e.to_string()),
                                            On::To(location),
                                        ));
                                    }
                                }
                            }
                            OrganismRequest::IntelligentMove(eyes) => {
                                match map.move_organism_with_eyes(&organism, eyes) {
                                    Ok(location) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::Moved,
                                            On::To(location),
                                        ))
                                    }
                                    Err(e) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::FailMoved(e.to_string()),
                                            On::None,
                                        ))
                                    }
                                }
                            }
                            OrganismRequest::EatFoodAround(location) => {
                                match map.feed_organism(&organism, location) {
                                    Ok(()) => {
                                        let o = organism.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::Ate,
                                            On::Food(location),
                                        ));
                                    }
                                    Err(e) => {
                                        let o = organism.read().unwrap();

                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::FailAte(e.to_string()),
                                            On::Food(location),
                                        ));
                                    }
                                }
                            }

                            OrganismRequest::KillAround(location) => {
                                let (mut killed, errors) = map.kill_around(&organism, location);
                                let o = organism.read().unwrap();
                                for kill in killed.iter() {
                                    let k = kill.read().unwrap();
                                    events.push(Event::new(
                                        self.lifetime,
                                        o.actor(),
                                        EventType::Killed,
                                        On::Actor(k.actor()),
                                    ));
                                }

                                for error in errors {
                                    events.push(Event::new(
                                        self.lifetime,
                                        o.actor(),
                                        EventType::FailKilled(error.to_string()),
                                        On::None,
                                    ));
                                }

                                dead_list.append(&mut killed);
                            }
                            OrganismRequest::Starve => match map.kill_organism(&organism) {
                                Ok(()) => {
                                    let o = organism.read().unwrap();
                                    events.push(Event::new(
                                        self.lifetime,
                                        Actor::Map,
                                        EventType::Starved,
                                        On::Actor(o.actor()),
                                    ));
                                    dead_list.push(Arc::clone(&organism))
                                }
                                Err(e) => {
                                    let org_info = organism.read().unwrap();

                                    events.push(Event::new(
                                        self.lifetime,
                                        Actor::Map,
                                        EventType::FailStarved(e.to_string()),
                                        On::Actor(org_info.actor()),
                                    ));

                                    //println!("Error killing organism! {}", e);
                                    errors.push(anyhow!(
                                        "Cannot starve organism: {:#?}\nerror: {}",
                                        org_info,
                                        e
                                    ))
                                }
                            },
                            OrganismRequest::Reproduce => {
                                if let Some(max_pop) = self.max_organisms {
                                    if self.organisms.len() > max_pop {
                                        continue;
                                    }
                                }
                                match map.deliver_child(&organism, self.settings.spawn_radius) {
                                    Ok(child) => {
                                        new_spawn.push(child);
                                    }
                                    Err(_e) => {
                                        let _organism = organism.read().unwrap();
                                        /*println!( "Error reproducing - Info:\norganism: {:#?}\n Error: {}",
                                        organism,
                                        e);
                                        */
                                        /*errors.push(anyhow!(
                                            "Error reproducing - Info:\norganism: {:#?}\n Error: {}",
                                            organism,
                                            e
                                        ));
                                        */
                                    }
                                }
                            }
                        }
                    }

                    (dead_list, new_spawn, errors, events)
                },
            )
        };
        self.events.append(&mut events);

        if !critical_errors.is_empty() {
            let mut error_msg = "Critical error(s) encountered:\n".to_string();
            for error in critical_errors {
                error_msg += &format!("{}\n", error);
            }
            return Err(anyhow!("{}", error_msg));
        }
        self.remove_dead(dead_list);

        self.organisms.append(&mut new_spawn);

        Ok(())
    }

    pub fn remove_dead(&mut self, list: Vec<Arc<RwLock<Organism>>>) {
        if !list.is_empty() {
            let uuid_list = list
                .into_iter()
                .map(|o| o.read().unwrap().id)
                .collect::<Vec<_>>();

            let (mut dead_organisms, alive_organisms, mut events) =
                self.organisms.clone().into_iter().fold(
                    (Vec::new(), Vec::new(), Vec::new()),
                    |(mut dead_organisms, mut alive_organisms, mut events), organism| {
                        let dead = {
                            let org_lock = organism.read().unwrap();
                            uuid_list.contains(&org_lock.id)
                        };

                        if dead {
                            //nothing should be holding onto this
                            dead_organisms.push(Arc::clone(&organism));
                            events.push(Event::new(
                                self.lifetime,
                                Actor::Map,
                                EventType::MovedToGraveyard,
                                On::Actor(organism.read().unwrap().actor()),
                            ));
                        } else {
                            alive_organisms.push(organism);
                        }

                        (dead_organisms, alive_organisms, events)
                    },
                );
            self.organisms = alive_organisms;

            self.graveyard.append(&mut dead_organisms);

            self.events.append(&mut events);
        }
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

#[derive(Debug)]
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
