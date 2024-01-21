use std::sync::{Arc, Mutex};

use crate::{Cell, Drawable, Organism};
use bevy::{
    ecs::system::{Commands, EntityCommands, Resource},
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
//mod threading;
//use threading::*;

///holds the map and organisms
#[derive(Resource)]
pub struct LEWorld {
    settings: WorldSettings,
    map: Mutex<WorldMap>,
    organisms: Vec<Arc<Mutex<Organism>>>,
    lifetime: u64,
    graveyard: Vec<Arc<Mutex<Organism>>>,
}

impl Default for LEWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl LEWorld {
    pub fn new() -> LEWorld {
        LEWorld {
            settings: WorldSettings::default(),
            map: Mutex::new(WorldMap::new()),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: Vec::new(),
        }
    }

    pub fn new_walled(length: u64) -> LEWorld {
        LEWorld {
            settings: WorldSettings::default(),
            map: Mutex::new(WorldMap::new_walled(length)),
            lifetime: 0,
            organisms: Vec::new(),
            graveyard: Vec::new(),
        }
    }

    pub fn add_simple_organism(&mut self, location: I64Vec3) {
        self.add_organism(Organism::new_simple(location));
    }
    pub fn add_organism(&mut self, organism: Organism) {
        let organism = Arc::new(Mutex::new(organism));

        self.insert_organism_into_map(&organism);

        self.organisms.push(organism);
    }

    pub fn insert_organism_into_map(&mut self, organism: &Arc<Mutex<Organism>>) {
        let mut map = self.map.lock().unwrap();
        if let Err(e) = map.insert_organism(organism) {
            println!("{}", e);
        }
    }

    pub fn tick(&mut self) -> Result<(), anyhow::Error> {
        println!(
            "tick {} - organism count: alive - {}, dead - {}",
            self.lifetime,
            self.organisms.len(),
            self.graveyard.len()
        );
        self.lifetime += 1;
        if self.organisms.is_empty() {
            return Err(anyhow!("everyone died!!!"));
        }
        let mut dead_list: Vec<usize> = Vec::new();
        let mut new_spawn: Vec<Arc<Mutex<Organism>>> = Vec::new();
        let organism_count = self.organisms.len();
        for (index, arc_organism) in self.organisms.iter_mut().enumerate() {
            let mut organism = arc_organism.lock().unwrap();
            let mut map = self.map.lock().unwrap();

            if dead_list.contains(&index) {
                continue;
            }
            if let Cell::Empty = map.get(organism.location) {
                dead_list.push(index);
                continue;
            }

            let requests = organism.tick(&map, &self.settings);

            let mut reverse_direction = false;

            for request in requests {
                match request {
                    WorldRequest::Food(location) => {
                        if let Err(_e) =
                            Self::try_gen_food(&mut map, self.settings.food_spawn_radius, location)
                        {
                            //do nothing
                            continue;
                        }
                    }
                    WorldRequest::MoveBy(location) => {
                        if let Err(e) =
                            Self::try_move_organism(&mut map, arc_organism, &mut organism, location)
                        {
                            println!("error: {}", e);
                            reverse_direction = true;
                            //do something
                            continue;
                        }
                    }
                    WorldRequest::EatFood(location) => {
                        if let Err(_e) = Self::try_eat(&mut map, &mut organism, location) {
                            //do something
                            continue;
                        }
                    }
                    WorldRequest::Kill(location) => {
                        match map.kill(location) {
                            Ok(_dead_organism) => {
                                println!("killed")
                                //we don't do anything here, because the dead organism is
                                //no longer in our map, so it's all fine.
                            }
                            Err(_e) => {
                                //do something
                            }
                        }
                    }
                    WorldRequest::Starve => {
                        if let Err(_e) = Self::try_starve(&mut map, &organism) {
                            //do something
                            continue;
                        };
                        dead_list.push(index);
                    }
                    WorldRequest::Reproduce => {
                        new_spawn.push(Arc::clone(arc_organism));
                    }
                }
            }
            if reverse_direction {
                organism.reverse_direction();
            }
        }
        if !dead_list.is_empty() {
            let (mut dead_organisms, alive_organisms) =
                self.organisms.clone().into_iter().enumerate().fold(
                    (Vec::new(), Vec::new()),
                    |mut acc, (index, org)| {
                        if dead_list.contains(&index) {
                            acc.0.push(org)
                        } else {
                            acc.1.push(org)
                        }
                        acc
                    },
                );
            self.organisms = alive_organisms;

            self.graveyard.append(&mut dead_organisms);
        }

        for spawn in new_spawn {
            let mut organism_to_clone = spawn.lock().unwrap();
            let Some(new_spawn) = organism_to_clone.reproduce() else {
                continue;
            };

            let mut map = self.map.lock().unwrap();

            let Ok(new_spawn_location) = map.get_valid_spawn_point(
                &new_spawn.organs,
                organism_to_clone.location,
                self.settings.spawn_radius,
            ) else {
                continue;
            };

            let new_organism = new_spawn.into_organism(new_spawn_location);

            let new_organism = Arc::new(Mutex::new(new_organism));

            map.insert_organism(&new_organism).unwrap();
            self.organisms.push(new_organism);
        }

        Ok(())
    }

    fn try_starve(map: &mut WorldMap, organism: &Organism) -> Result<(), anyhow::Error> {
        organism
            .occupied_locations()
            .for_each(|location| map.replace(location, Cell::Food));
        Ok(())
    }

    fn try_kill(map: &mut WorldMap, kill: I64Vec3) -> Result<(), anyhow::Error> {
        map.kill(kill)
    }

    fn try_eat(
        map: &mut WorldMap,
        organism: &mut Organism,
        eat: I64Vec3,
    ) -> Result<(), anyhow::Error> {
        let mut can_eat = true;
        match map.get(eat) {
            Cell::Food => {}
            _ => can_eat = false,
        }
        if !can_eat {
            return Err(anyhow!("Cannot eat!"));
        }

        organism.feed(1);

        map.clear(eat);

        Ok(())
    }

    //arc_organism is for cloning
    //organism is used for the locations of the organism's organs.
    fn try_move_organism(
        map: &mut WorldMap,
        arc_organism: &Arc<Mutex<Organism>>,
        organism_info: &mut Organism,
        move_by: I64Vec3,
    ) -> Result<(), anyhow::Error> {
        //validate that the locations it wants to move to are unoccupied
        let mut can_move = true;
        for location in organism_info.occupied_locations() {
            #[allow(clippy::single_match)]
            match map.get(location + move_by) {
                Cell::Wall => {
                    can_move = false;
                    break;
                }
                _ => {}
            }
        }

        if !can_move {
            return Err(anyhow!("Can't move organism to new location!"));
        }

        for (location, organ) in organism_info.organs() {
            let cell = map.get(location + move_by);
            *cell = Cell::organism(arc_organism, organ);
            map.clear(location);
        }

        organism_info.move_by(move_by);

        Ok(())
    }

    fn try_gen_food(
        map: &mut WorldMap,
        radius: i64,
        location: I64Vec3,
    ) -> Result<(), anyhow::Error> {
        let mut rng = rand::thread_rng();

        let mut x = rng.gen_range(-radius..=radius);
        let mut y = rng.gen_range(-radius..=radius);
        if x == 0 && y == 0 {
            if rng.gen::<bool>() {
                x = if rng.gen::<bool>() { 1 } else { -1 };
            } else {
                y = if rng.gen::<bool>() { 1 } else { -1 };
            }
        }

        let random_spot = location + I64Vec3::new(x, y, 0);

        let mut attempts = 0;
        loop {
            match map.get(random_spot) {
                Cell::Empty => {
                    let food_cell = map.get(random_spot);
                    *food_cell = Cell::Food;
                    return Ok(());
                }
                _ => {
                    attempts += 1;
                }
            }

            if attempts == 3 {
                return Err(anyhow!(
                    "Could not spawn food after three randomized attempts!"
                ));
            }
        }
    }

    pub fn draw(&self, commands: &mut Commands) -> Vec<SpriteBundle> {
        let map = self.map.lock().unwrap();

        let mut sprites: Vec<SpriteBundle> = Vec::with_capacity(self.organisms.len());

        for (location, square) in map.iter() {
            match square {
                Cell::Empty => {}
                _ => {
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
            }
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

    world.add_simple_organism((0, 0, 0).into());
}

#[test]
fn create_world_panic() {
    let mut world = LEWorld::new();
    world.add_simple_organism((0, 0, 0).into());
    world.add_simple_organism((0, 0, 0).into());
}
