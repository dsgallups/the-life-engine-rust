use std::ops::Add;

use bevy::{prelude::*, utils::HashMap};
use rand::{seq::SliceRandom as _, thread_rng, Rng as _};

use crate::{cell::CellType, organism::genome::CellLocation, ORGANISM_LAYER};

#[derive(Component, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct GlobalCellLocation {
    x: i32,
    y: i32,
}

impl GlobalCellLocation {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add_x(self, add: i32) -> Self {
        Self {
            x: self.x + add,
            y: self.y,
        }
    }
    pub fn add_y(self, add: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + add,
        }
    }

    pub fn around(&self) -> [GlobalCellLocation; 4] {
        let mut arr = [self.add_x(1), self.add_y(1), self.add_x(-1), self.add_y(-1)];
        let mut rng = thread_rng();
        arr.shuffle(&mut rng);
        arr
    }

    /// returns a random location within the radius (actually it's a square rn cuz im lazy)
    pub fn rand_around(&self, radius: u16) -> GlobalCellLocation {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-(radius as i32)..=(radius as i32));
        let y = rng.gen_range(-(radius as i32)..=(radius as i32));

        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, ORGANISM_LAYER)
    }
}

impl Add<CellLocation> for GlobalCellLocation {
    type Output = GlobalCellLocation;
    fn add(self, rhs: CellLocation) -> Self::Output {
        GlobalCellLocation {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Resource, Default)]
/// the value should only contain organism entities (with the organism cell type associated)
/// and then entities of food.
///
/// the catch is that an organism is not itself a cell type, but a collection of cells and all
///
/// so yeah.  
pub struct OccupiedLocations(HashMap<GlobalCellLocation, (Entity, CellType)>);

impl OccupiedLocations {
    pub fn insert(
        &mut self,
        location: GlobalCellLocation,
        occupier: Entity,
        cell_type: impl Into<CellType>,
    ) -> Option<(Entity, CellType)> {
        self.0.insert(location, (occupier, cell_type.into()))
    }

    pub fn occupied(&self, location: &GlobalCellLocation) -> bool {
        self.0.get(location).is_some()
    }

    pub fn remove(&mut self, location: &GlobalCellLocation) -> Option<(Entity, CellType)> {
        self.0.remove(location)
    }

    pub fn get(&self, location: &GlobalCellLocation) -> Option<(Entity, CellType)> {
        self.0.get(location).cloned()
    }

    pub fn cell_type_at(&self, location: &GlobalCellLocation) -> Option<CellType> {
        self.0.get(location).map(|val| val.1)
    }
    #[allow(dead_code)]
    pub fn entity_at(&self, location: &GlobalCellLocation) -> Option<Entity> {
        self.0.get(location).map(|val| val.0)
    }
}
