use std::{
    sync::{Arc, RwLock},
    thread,
};

use crate::{Cell, Drawable, Event, EventType, On, Organism};
use bevy::{
    ecs::system::{Commands, Resource},
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
mod map;
pub use map::*;
use rustc_hash::FxHashSet;
#[cfg(feature = "log")]
use uuid::Uuid;

///holds the map and organisms
#[derive(Resource, Debug)]
#[allow(dead_code)]
pub struct LEWorld {
    settings: Arc<WorldSettings>,
    map: RwLock<WorldMap>,
    organisms: Vec<Arc<RwLock<Organism>>>,
    lifetime: u64,
    graveyard: Vec<Arc<RwLock<Organism>>>,
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
            settings: Arc::clone(&settings),
            map: RwLock::new(WorldMap::new(&settings)),
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
        let settings = Arc::new(settings);

        let thread_count = thread::available_parallelism().unwrap().get() as u64;
        LEWorld {
            settings: Arc::clone(&settings),
            map: RwLock::new(WorldMap::new(&settings)),
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

    pub fn add_simple_producer(&mut self, location: I64Vec2, commands: &mut Commands) {
        self.add_organism(Organism::simple_producer(location, commands));
    }
    pub fn add_simple_mover(&mut self, location: I64Vec2, commands: &mut Commands) {
        self.add_organism(Organism::simple_mover(location, commands));
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

        //self.add_simple_producer((0, 0).into());
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
        let mut new_settings = self.settings.as_ref().clone();
        new_settings.max_organisms = population;
        let new_settings = Arc::new(new_settings);
        let mut map = self.map.write().unwrap();
        map.set_settings(&new_settings);
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
            let o = organism.read().unwrap();
            self.log_id(o.id);
        }
    }

    pub fn check_alive(&self) {
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

        #[cfg(feature = "log")]
        if self.organisms.len() > live_organisms_in_map.len() {
            println!("AT LEAST ONE IS MISSING:");
            for organism in self.organisms.iter() {
                let o = organism.read().unwrap();
                if !live_organisms_in_map.contains(&o.id) {
                    self.log_id(o.id);
                }
            }
        }
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

        let Some(_id) = id else {
            return;
        };
        #[cfg(feature = "log")]
        self.log_id(_id);
    }

    pub fn log(&self) {
        println!("World Information:");
        println!("Alive Organisms: {:?}", self.organisms);
    }

    pub fn tick(&mut self, mut commands: &mut Commands) -> Result<(), anyhow::Error> {
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
            .map(|arc_organism| {
                let mut organism_lock = arc_organism.write().unwrap();
                let requests = organism_lock.tick(&self.settings);
                (Arc::clone(arc_organism), requests)
            })
            .collect::<Vec<_>>();

        let (dead_list, mut new_spawn, _critical_errors, mut _events) = {
            let mut map = self.map.write().unwrap();
            requests.into_iter().fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
                |(mut dead_list, mut new_spawn, mut _errors, mut events), (organism, requests)| {
                    for request in requests {
                        match request {
                            OrganismRequest::ProduceFoodAround(location) => {
                                match map.produce_food_around(location) {
                                    Ok(()) => {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::Produced,
                                            On::Around(location),
                                        ));
                                    }
                                    Err(_e) => {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::FailProduced(_e.to_string()),
                                            On::Around(location),
                                        ));
                                    }
                                }
                            }
                            OrganismRequest::MoveBy(location) => {
                                match map.move_organism(&organism, location) {
                                    Ok(()) =>
                                    {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::Moved,
                                            On::To(location),
                                        ))
                                    }
                                    Err(_e) => {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::FailMoved(_e.to_string()),
                                            On::To(location),
                                        ));
                                    }
                                }
                            }
                            OrganismRequest::IntelligentMove(eyes) => {
                                match map.move_organism_with_eyes(&organism, eyes) {
                                    Ok(_location) =>
                                    {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::Moved,
                                            On::To(_location),
                                        ))
                                    }
                                    Err(_e) =>
                                    {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::FailMoved(_e.to_string()),
                                            On::None,
                                        ))
                                    }
                                }
                            }
                            OrganismRequest::EatFoodAround(location) => {
                                match map.feed_organism(&organism, location) {
                                    Ok(_amount) =>
                                    {
                                        #[cfg(feature = "log")]
                                        if _amount > 0 {
                                            events.push(Event::new(
                                                self.lifetime,
                                                organism.read().unwrap().actor(),
                                                EventType::Ate,
                                                On::Food(location, _amount),
                                            ));
                                        }
                                    }
                                    Err(_e) => {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::FailAte(_e.to_string()),
                                            On::Food(location, 0),
                                        ));
                                    }
                                }
                            }

                            OrganismRequest::KillAround(location) => {
                                let mut killed = if cfg!(feature = "log") {
                                    let (killed, errors) = map.kill_around(&organism, location);
                                    let o = organism.read().unwrap();
                                    for kill in killed.iter() {
                                        let k = kill.read().unwrap();
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
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
                                    killed
                                } else {
                                    let (killed, _) = map.kill_around(&organism, location);
                                    killed
                                };

                                dead_list.append(&mut killed);
                            }
                            OrganismRequest::Starve => match map.kill_organism(&organism) {
                                Ok(()) => {
                                    #[cfg(feature = "log")]
                                    events.push(Event::new(
                                        self.lifetime,
                                        Actor::Map,
                                        EventType::Starved,
                                        On::Actor(organism.read().unwrap().actor()),
                                    ));
                                    dead_list.push(Arc::clone(&organism))
                                }
                                Err(_e) => {
                                    #[cfg(feature = "log")]
                                    events.push(Event::new(
                                        self.lifetime,
                                        Actor::Map,
                                        EventType::FailStarved(_e.to_string()),
                                        On::Actor(organism.read().unwrap().actor()),
                                    ));

                                    _errors.push(organism.read().unwrap().id);
                                }
                            },
                            OrganismRequest::Reproduce => {
                                if let Some(max_pop) = self.settings.max_organisms {
                                    if self.organisms.len() >= max_pop {
                                        let mut o = organism.write().unwrap();
                                        //it shouldn't hold onto the food it has
                                        let _ = o.reproduce(commands);

                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            o.actor(),
                                            EventType::FailReproduced(
                                                "max pop exceeded".to_string(),
                                            ),
                                            On::None,
                                        ));
                                        continue;
                                    }
                                }
                                match map.deliver_child(&organism, self.settings.spawn_radius) {
                                    Ok(child) => {
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::Reproduced,
                                            On::Actor(child.read().unwrap().actor()),
                                        ));
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
                                        #[cfg(feature = "log")]
                                        events.push(Event::new(
                                            self.lifetime,
                                            organism.read().unwrap().actor(),
                                            EventType::FailReproduced(_e.to_string()),
                                            On::None,
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    (dead_list, new_spawn, _errors, events)
                },
            )
        };
        #[cfg(feature = "log")]
        self.events.append(&mut _events);

        //append new spawn first, so if they're killed in the same tick, they are removed
        self.organisms.append(&mut new_spawn);

        self.remove_dead(dead_list);
        #[cfg(feature = "log")]
        if !_critical_errors.is_empty() {
            let mut error_msg = "Critical error(s) encountered:\n".to_string();
            for error in _critical_errors {
                error_msg += &format!("{}\n", error);
                self.log_id(error);
            }
            return Err(anyhow!("{}", error_msg));
        }

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
                let organism = organism.read().unwrap();
                organism_list.push(organism.id);
            }

            let (mut dead_organisms, alive_organisms, mut _events) =
                self.organisms.clone().into_iter().fold(
                    (Vec::new(), Vec::new(), Vec::<Event>::new()),
                    |(mut dead_organisms, mut alive_organisms, mut _events), organism| {
                        let dead = {
                            let org_lock = organism.read().unwrap();
                            uuid_list.contains(&org_lock.id)
                        };

                        if dead {
                            //nothing should be holding onto this
                            dead_organisms.push(Arc::clone(&organism));
                            #[cfg(feature = "log")]
                            _events.push(Event::new(
                                self.lifetime,
                                Actor::Map,
                                EventType::MovedToGraveyard,
                                On::Actor(organism.read().unwrap().actor()),
                            ));
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

    //world.add_simple_producer((0, 0).into());
}

#[test]
fn create_world_panic() {
    let mut world = LEWorld::new();
    //world.add_simple_producer((0, 0).into());
    //world.add_simple_producer((0, 0).into());
}
