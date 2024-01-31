use bevy::{ecs::system::Resource, math::I64Vec2};
use rustc_hash::FxHashMap;

use crate::{neighbors::NEIGHBORS, Cell, Organism};

#[derive(Resource, Default)]
pub struct WorldMap(FxHashMap<I64Vec2, Cell>);

impl WorldMap {
    pub fn new() -> Self {
        Self(FxHashMap::default())
    }

    pub fn insert_organism(&mut self, organism: Organism) {
        todo!();
    }

    pub fn insert_food_around(&self, location: I64Vec2) -> Option<(I64Vec2, Cell)> {
        for neighbor in NEIGHBORS.adjacent {
            let new_location = location + neighbor;

            if !self.0.contains_key(&new_location) {
                self.0.insert(new_location, Cell::Food);
                return Some((new_location, Cell::Food));
            }
        }

        None
    }
}
