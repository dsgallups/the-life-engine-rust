use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::Organism;
use bevy::math::I64Vec3;
use rustc_hash::FxHashMap;

mod square;
//mod threading;
//use threading::*;

///holds the map and organisms
pub struct LEWorld {
    settings: WorldSettings,
    map: FxHashMap<I64Vec3, Option<Arc<Mutex<Organism>>>>,
    organisms: Vec<Arc<Mutex<Organism>>>,
}

impl Default for LEWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl LEWorld {
    pub fn new() -> LEWorld {
        let thread = thread::spawn(move || {});

        LEWorld {
            settings: WorldSettings::default(),
            map: FxHashMap::default(),
            organisms: Vec::new(),
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
        organism
            .lock()
            .unwrap()
            .occupied_locations()
            .for_each(|location| {
                if self.map.get(&location).is_none() {
                    self.map.insert(location, Some(organism.clone()));
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
    pub fn tick(&mut self) {
        for organism in self.organisms.iter_mut() {
            let organism = organism.lock().unwrap();
        }
    }
}

pub struct WorldSettings {
    food_spawn_radius: u64,
    producer_threshold: u8,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            food_spawn_radius: 1,
            producer_threshold: 10,
        }
    }
}

#[test]
fn create_world() {
    let mut world = LEWorld::new();

    world.add_simple_organism((0, 0, 1).into());
}

#[test]
fn create_world_panic() {
    let mut world = LEWorld::new();
    world.add_simple_organism((0, 0, 1).into());
    world.add_simple_organism((0, 0, 1).into());
}
