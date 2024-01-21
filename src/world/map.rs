use std::{
    collections::hash_map::Iter,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use bevy::math::I64Vec3;
use rand::Rng;
use rustc_hash::FxHashMap;

use crate::{Cell, Organ, Organism};

pub struct WorldMap {
    squares: FxHashMap<I64Vec3, Cell>,
}

impl Default for WorldMap {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldMap {
    pub fn new() -> Self {
        Self {
            squares: FxHashMap::default(),
        }
    }

    pub fn new_walled(length: u64) -> Self {
        let mut squares = FxHashMap::default();

        let half = (length / 2) as i64;

        for i in -half..half {
            for j in -2..=2 {
                let location = I64Vec3::new(i, -half + j, 0);
                squares.insert(location, Cell::Wall);
                let location = I64Vec3::new(i, half + j, 0);
                squares.insert(location, Cell::Wall);
                let location = I64Vec3::new(-half + j, i, 0);
                squares.insert(location, Cell::Wall);
                let location = I64Vec3::new(half + j, i, 0);
                squares.insert(location, Cell::Wall);
            }
        }

        Self { squares }
    }

    pub fn get(&mut self, location: I64Vec3) -> &mut Cell {
        self.squares.entry(location).or_default()
    }

    pub fn kill(&mut self, location: I64Vec3) -> Result<(), anyhow::Error> {
        let Some(Cell::Organism(organism_to_kill, _)) = self.squares.get(&location) else {
            return Err(anyhow!("Cannot kill!"));
        };

        let organism_to_kill = Arc::clone(organism_to_kill);

        let organism_to_kill = organism_to_kill.lock().unwrap();

        for location in organism_to_kill.occupied_locations() {
            let res = self.squares.entry(location).or_default();
            *res = Cell::Food;
        }
        Ok(())
    }

    pub fn check(&self, location: &I64Vec3) -> Option<&Cell> {
        self.squares.get(location)
    }

    pub fn clear(&mut self, location: I64Vec3) {
        let res = self.squares.entry(location).or_default();
        *res = Cell::Empty;
    }
    pub fn replace(&mut self, location: I64Vec3, block: Cell) {
        let res = self.squares.entry(location).or_default();
        *res = block;
    }
    pub fn iter(&self) -> Iter<'_, I64Vec3, Cell> {
        self.squares.iter()
    }

    //todo(dsgallups):
    pub fn get_valid_spawn_point(
        &self,
        organs: &[Organ],
        basis: I64Vec3,
        deviate_by: u64,
    ) -> Result<I64Vec3, anyhow::Error> {
        let mut rng = rand::thread_rng();

        let mut attempt_count = 0;
        loop {
            let x = rng.gen_range(-(deviate_by as i64)..=deviate_by as i64);
            let y = rng.gen_range(-(deviate_by as i64)..=deviate_by as i64);
            let new_basis = basis + I64Vec3::new(x, y, 0);

            let mut valid_basis = true;

            for organ in organs {
                match self.squares.get(&(organ.relative_location + new_basis)) {
                    None => {}
                    Some(Cell::Empty) => {}
                    _ => {
                        valid_basis = false;
                    }
                }
            }
            if valid_basis {
                return Ok(new_basis);
            }
            attempt_count += 1;
            if attempt_count == 10 {
                return Err(anyhow!("couldn't find place to put down organism"));
            }
        }
    }

    pub fn insert_organism(
        &mut self,
        organism: &Arc<Mutex<Organism>>,
    ) -> Result<(), anyhow::Error> {
        let org_lock = organism.lock().unwrap();

        //check to see if any of the locations are occupied
        for (location, _organ) in org_lock.organs() {
            match self.get(location) {
                Cell::Empty => {}
                _ => {
                    return Err(anyhow!(
                        "cannot insert an organism into an occupied square!"
                    ))
                }
            }
        }

        for (location, organ) in org_lock.organs() {
            let cell = self.get(location);
            match cell {
                Cell::Empty => {
                    *cell = Cell::organism(organism, organ);
                }
                _ => {
                    return Err(anyhow!(
                        "cannot insert an organism into an occupied square!"
                    ))
                }
            }
        }
        Ok(())
    }

    pub fn get_food_around(&self, location: I64Vec3) -> Option<Vec<I64Vec3>> {
        let mut food_locations = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let looking_at = location + I64Vec3::new(i, j, 0);

                match self.squares.get(&looking_at) {
                    Some(Cell::Food) => food_locations.push(looking_at),
                    Some(_) => {}
                    None => {}
                };
            }
        }
        if food_locations.is_empty() {
            None
        } else {
            Some(food_locations)
        }
    }
}

impl<'a> IntoIterator for &'a WorldMap {
    type Item = (&'a I64Vec3, &'a Cell);
    type IntoIter = <&'a std::collections::HashMap<
        bevy::math::I64Vec3,
        Cell,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter()
    }
}
impl<'a> IntoIterator for &'a mut WorldMap {
    type Item = (&'a I64Vec3, &'a mut Cell);
    type IntoIter = <&'a mut std::collections::HashMap<
        bevy::math::I64Vec3,
        Cell,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter_mut()
    }
}

impl IntoIterator for WorldMap {
    type Item = (I64Vec3, Cell);
    type IntoIter = <std::collections::HashMap<
        bevy::math::I64Vec3,
        Cell,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.into_iter()
    }
}
