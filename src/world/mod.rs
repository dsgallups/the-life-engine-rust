use std::{
    collections::hash_map::Entry,
    sync::{Arc, Mutex},
};

use crate::{Cell, Drawable, Organism};
use bevy::{
    ecs::system::{Commands, Resource},
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
        organism
            .lock()
            .unwrap()
            .organs()
            .for_each(|(location, organ)| {
                if map.get(&location).is_none() {
                    map.insert(location, Cell::organism(organism, organ));
                } else {
                    panic!(
                        "attempted to insert organism into a location that is already occupied!"
                    );
                }
            });
    }

    pub fn refresh_map(&mut self) {
        /*for organism in self.organisms.iter() {
            let position = &organism.location;
            for organ in organism.organs.iter() {
                println!("organ_position = {:?}", organ.relative_location);
                println!("position = {:?}", position);
                let position = (organ.relative_location + (*position).as_i64vec3()).as_u64vec3();
                println!("position = {:?}", position);
                self.map[position.x as usize][position.y as usize] =
                    Cell::Organism(organ.cell.clone());
            }
        }*/
    }

    /// world will provide the organism with a request for its context requirements
    /// given the requirements provided by the organism, the world will provide the organism with the information it knows
    /// the organism will then provide the world with a request to update the world
    /// the world will then provide the organism with a response to the request, as its request may not always be fulfilled
    /**
     * Example 1
     *
     * let requested_context = organism.context_request();
     *
     * let OrganismContextRequest { nearest_food } = requested_context;
     *
     * let context_response = if !nearest_food {
     *      WorldContextResponse { nearest_food: None }
     * } else {
     *      let mut nearest_food_loc = I64Vec3::MAX;
     *      let position = organism.origin();
     *      let mut nearest_distance = std::u64::MAX;
     *      for (x, col) in self.map.iter().enumerate() {
     *          for (y, cell) in col.iter().enumerate() {
     *              if let Cell::Inert(InertCell::Food) = cell {
     *                  let distance = (position.x - x as u64).pow(2) + (position.y - y as u64).pow(2);
     *                      if distance < nearest_distance {
     *                          let x = x as i64 - position.x as i64;
     *                          let y = y as i64 - position.y as i64;
     *                          nearest_distance = distance;
     *                          nearest_food_loc = (x, y, 1).into();
     *                      }
     *                  }
     *              }
     *          }
     *     }
     *      WorldContextResponse {
     *          nearest_food: Some(nearest_food_loc),
     *      }
     * };
     *
     * let _requested_update = organism.update_request(context_response);
     *
     * let response = WorldUpdateResponse {};
     *
     * organism.tick(response);
     */
    pub fn tick(&mut self) -> Result<(), anyhow::Error> {
        let mut dead_list: Vec<usize> = Vec::new();
        for (index, arc_organism) in self.organisms.iter_mut().enumerate() {
            let mut organism = arc_organism.lock().unwrap();
            let mut map = self.map.lock().unwrap();

            if map.get(&organism.location).is_none() {
                //this organism was killed
                dead_list.push(index);
                continue;
            }
            let requests = organism.tick(&map, &self.settings);

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
                        if let Err(_e) =
                            Self::try_move_organism(&mut map, arc_organism, &mut organism, location)
                        {
                            //do something
                            continue;
                        }
                    }
                    WorldRequest::EatFood(location) => {
                        println!("\n\n\neat food request!!\n\n\n");
                        if let Err(_e) = Self::try_eat(&mut map, &mut organism, location) {
                            //do something
                            continue;
                        }
                    }
                    WorldRequest::Kill(location) => {
                        match Self::try_kill(&mut map, &mut organism, location) {
                            Ok(_dead_organism) => {
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
                }
            }
        }

        for index in dead_list {
            let dead_organism = self.organisms.swap_remove(index);
            self.graveyard.push(dead_organism);
        }

        let map = self.map.lock().unwrap();
        println!("map:\n");
        for square in map.iter() {
            println!("{:?}: {:?}", square.0, square.1);
        }

        Ok(())
    }

    fn try_starve(map: &mut WorldMap, organism: &Organism) -> Result<(), anyhow::Error> {
        organism.occupied_locations().for_each(|location| {
            map.remove(&location);
        });
        Ok(())
    }

    fn try_kill(
        map: &mut WorldMap,
        organism: &mut Organism,
        kill: I64Vec3,
    ) -> Result<Arc<Mutex<Organism>>, anyhow::Error> {
        let Some(Cell::Organism(organism_to_kill, _)) = map.get(&kill) else {
            return Err(anyhow!("Cannot kill!"));
        };
        let organism_to_kill_arc = Arc::clone(organism_to_kill);
        let organism_to_kill = organism_to_kill_arc.lock().unwrap();

        //clear all the squares of the organism
        let mut feed_count = 0;
        for location in organism_to_kill.occupied_locations() {
            feed_count += 1;
            map.remove(&location);
        }

        organism.feed(feed_count);
        drop(organism_to_kill);

        Ok(organism_to_kill_arc)
    }

    fn try_eat(
        map: &mut WorldMap,
        organism: &mut Organism,
        eat: I64Vec3,
    ) -> Result<(), anyhow::Error> {
        let mut can_eat = true;
        match map.get(&eat) {
            Some(Cell::Food) => {}
            Some(_) => can_eat = false,
            None => can_eat = false,
        }
        if !can_eat {
            return Err(anyhow!("Cannot eat!"));
        }

        organism.feed(1);

        map.remove(&eat);

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
            if map.get(&(location + move_by)).is_some() {
                can_move = false;
                break;
            }
        }

        if !can_move {
            return Err(anyhow!("Can't move organism to new location!"));
        }

        for (location, organ) in organism_info.organs() {
            map.insert(location + move_by, Cell::organism(arc_organism, organ));
            map.remove(&location);
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
        if x == 0 {
            x = if rng.gen::<bool>() { 1 } else { -1 };
        }
        if y == 0 {
            y = if rng.gen::<bool>() { 1 } else { -1 };
        }
        let random_spot = location + I64Vec3::new(x, y, 0);

        let mut attempts = 0;
        loop {
            match map.entry(random_spot) {
                Entry::Occupied(_) => {
                    attempts += 1;
                }
                Entry::Vacant(_) => {
                    map.insert(random_spot, Cell::Food);
                    return Ok(());
                }
            };
            if attempts == 3 {
                return Err(anyhow!(
                    "Could not spawn food after three randomized attempts!"
                ));
            }
        }
    }

    pub fn draw(&self, commands: &mut Commands) {
        println!("in draw");
        let map = self.map.lock().unwrap();

        for (location, square) in map.iter() {
            let color = square.color();
            commands.spawn(SpriteBundle {
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

pub struct WorldSettings {
    pub food_spawn_radius: i64,
    pub producer_threshold: u8,
    //every nth tick of an organism being alive, decrease its food consumed by 1
    pub hunger_tick: u64,
    pub reproduce_at: u64,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            food_spawn_radius: 1,
            hunger_tick: 6,
            producer_threshold: 2,
            reproduce_at: 3,
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
