use std::{
    collections::hash_map::{Entry, Iter},
    sync::{Arc, RwLock},
};

use anyhow::anyhow;
use bevy::math::I64Vec3;
use rand::Rng;
use rustc_hash::FxHashMap;
use uuid::Uuid;

use crate::{Cell, OrganType, Organism};

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

    pub fn get(&self, location: &I64Vec3) -> Option<&Cell> {
        self.squares.get(location)
    }

    pub fn feed_organism(
        &mut self,
        organism: &Arc<RwLock<Organism>>,
        location: I64Vec3,
    ) -> Result<(), anyhow::Error> {
        let mut consumed = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let looking_at = location + I64Vec3::new(i, j, 0);

                match self.squares.get(&looking_at) {
                    Some(Cell::Food) => {
                        self.squares.remove(&looking_at);
                        consumed += 1;
                    }
                    Some(_) => {}
                    None => {}
                };
            }
        }

        let mut org_lock = organism.write().unwrap();
        org_lock.feed(consumed);

        Ok(())
    }
    pub fn kill_around(
        &mut self,
        killer: &Arc<RwLock<Organism>>,
        killer_organ_location: I64Vec3,
    ) -> Vec<Uuid> {
        let mut kill_list = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let looking_at = killer_organ_location + I64Vec3::new(i, j, 0);

                match self.squares.get(&looking_at) {
                    Some(Cell::Organism(organism, organ)) => {
                        let organism_lock = organism.read().unwrap();
                        let killer_lock = killer.read().unwrap();
                        if killer_lock.id == organism_lock.id {
                            continue;
                        }
                        let organ_lock = organ.read().unwrap();
                        if organ_lock.r#type == OrganType::Armor {
                            continue;
                        }
                        kill_list.push(Arc::clone(organism));
                    }
                    Some(_) => {}
                    None => {}
                };
            }
        }

        let mut dead_organisms: Vec<Uuid> = Vec::with_capacity(kill_list.len());

        for organism in kill_list {
            match self.kill_organism(&organism) {
                Ok(dead_id) => dead_organisms.push(dead_id),
                Err(_e) => {
                    //most likely the organism is already dead, do nothing
                }
            }
        }
        dead_organisms
    }

    pub fn kill_organism(
        &mut self,
        dead_organism: &Arc<RwLock<Organism>>,
    ) -> Result<Uuid, anyhow::Error> {
        let (id, dead_organism_location) = {
            let lock = dead_organism.read().unwrap();

            (lock.id, lock.location)
        };

        let locations_to_remove: Vec<I64Vec3> = {
            let Some(Cell::Organism(organism, _organ)) = self.get(&dead_organism_location) else {
                return Err(anyhow!(
                    "An organism doesn't exist at this location anymore!"
                ));
            };

            let org_lock = organism.read().unwrap();
            org_lock.occupied_locations().collect()
        };

        for location in locations_to_remove {
            if let None = self.insert(location, Cell::Food) {
                panic!("somehow, the organism is not in locations to remove!");
            };
        }

        Ok(id)
    }

    pub fn move_organism(
        &mut self,
        organism: &Arc<RwLock<Organism>>,
        move_by: I64Vec3,
    ) -> Result<(), anyhow::Error> {
        let can_move = {
            let mut can_move = true;
            let org_lock = organism.read().unwrap();

            for location in org_lock.occupied_locations() {
                match self.get(&(location + move_by)) {
                    None => {}
                    Some(Cell::Food) => {}
                    _ => {
                        can_move = false;
                        break;
                    }
                }
            }
            can_move
        };

        let mut org_lock = organism.write().unwrap();
        if !can_move {
            org_lock.collide();
            return Err(anyhow!("Can't move organism to new location!"));
        }

        for (location, organ) in org_lock.arc_organs() {
            self.insert(location + move_by, Cell::organism(&organism, organ));

            self.remove(location);
        }

        org_lock.move_by(move_by);
        Ok(())
    }

    pub fn produce_food_around(
        &mut self,
        location: I64Vec3,
        radius: i64,
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
            match self.get(&random_spot) {
                None => {
                    self.insert(random_spot, Cell::Food);
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

    pub fn deliver_child(
        &mut self,
        parent: &Arc<RwLock<Organism>>,
        spawn_radius: u64,
    ) -> Result<Arc<RwLock<Organism>>, anyhow::Error> {
        let mut parent = parent.write().unwrap();

        let Some(new_spawn) = parent.reproduce() else {
            return Err(anyhow!("The parent failed to produce a baby"));
        };

        let mut rng = rand::thread_rng();
        let basis = parent.location;

        let mut attempt_count = 0;
        let baby_location: I64Vec3 = loop {
            let x = rng.gen_range(-(spawn_radius as i64)..=spawn_radius as i64);
            let y = rng.gen_range(-(spawn_radius as i64)..=spawn_radius as i64);
            let new_basis = basis + I64Vec3::new(x, y, 0);

            let mut valid_basis = true;

            for (_abs, organ) in parent.organs() {
                match self.squares.get(&(new_basis + organ.relative_location)) {
                    None => {}
                    _ => {
                        println!("item found at {}", new_basis + organ.relative_location);
                        valid_basis = false;
                    }
                }
            }
            if valid_basis {
                break new_basis;
            }
            attempt_count += 1;
            if attempt_count == 10 {
                return Err(anyhow!(
                    "couldn't find place to put down organism. spawn radius is {}",
                    spawn_radius
                ));
            }
        };

        let new_organism = Arc::new(RwLock::new(new_spawn.into_organism(baby_location)));

        self.insert_organism(&new_organism).unwrap();

        Ok(new_organism)
    }

    pub fn check(&self, location: &I64Vec3) -> Option<&Cell> {
        self.squares.get(location)
    }

    pub fn remove(&mut self, location: I64Vec3) -> Option<Cell> {
        self.squares.remove(&location)
    }
    pub fn insert(&mut self, location: I64Vec3, block: Cell) -> Option<Cell> {
        self.squares.insert(location, block)
    }

    pub fn iter(&self) -> Iter<'_, I64Vec3, Cell> {
        self.squares.iter()
    }

    pub fn insert_organism(
        &mut self,
        organism: &Arc<RwLock<Organism>>,
    ) -> Result<(), anyhow::Error> {
        let org_lock = organism.read().unwrap();

        //check to see if any of the locations are occupied
        for (location, _organ) in org_lock.organs() {
            if self.get(&location).is_some() {
                return Err(anyhow!(
                    "cannot insert an organism into an occupied square! location in question: {}",
                    location
                ));
            }
        }

        for (location, organ) in org_lock.arc_organs() {
            match self.squares.entry(location) {
                Entry::Occupied(_) => {
                    return Err(anyhow!(
                        "cannot insert an organism into an occupied square!"
                    ))
                }
                Entry::Vacant(e) => {
                    e.insert(Cell::organism(organism, organ));
                }
            }
        }
        Ok(())
    }

    //this doesn't account for the organism of the square that calls this
    pub fn get_organisms_touching(&self, location: I64Vec3) -> Option<Vec<I64Vec3>> {
        let mut touching_organisms = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let check = location + I64Vec3::new(i, j, 0);

                match self.squares.get(&check) {
                    Some(Cell::Organism(_organism, _organ)) => {
                        touching_organisms.push(check);
                    }
                    Some(_) => {}
                    None => {}
                };
            }
        }

        if touching_organisms.is_empty() {
            None
        } else {
            Some(touching_organisms)
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
