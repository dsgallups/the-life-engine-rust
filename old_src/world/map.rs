use std::{
    collections::hash_map::{Entry, Iter},
    sync::{Arc, RwLock},
};

use anyhow::anyhow;
use bevy::math::I64Vec2;
use rand::Rng;
use rustc_hash::FxHashMap;

use crate::{Cell, Direction, OrganType, Organism, WorldSettings};

#[derive(Debug)]
pub struct WorldMap {
    settings: Arc<WorldSettings>,
    squares: FxHashMap<I64Vec2, Cell>,
}

impl WorldMap {
    pub fn new(settings: &Arc<WorldSettings>) -> Self {
        let mut squares = FxHashMap::default();

        if let Some(half) = settings.wall_length_half {
            for i in -half..half {
                for j in -2..=2 {
                    let location = I64Vec2::new(i, -half + j);
                    squares.insert(location, Cell::Wall);
                    let location = I64Vec2::new(i, half + j);
                    squares.insert(location, Cell::Wall);
                    let location = I64Vec2::new(-half + j, i);
                    squares.insert(location, Cell::Wall);
                    let location = I64Vec2::new(half + j, i);
                    squares.insert(location, Cell::Wall);
                }
            }
        }

        Self {
            settings: Arc::clone(settings),
            squares,
        }
    }

    pub fn set_settings(&mut self, settings: &Arc<WorldSettings>) {
        self.settings = Arc::clone(settings);
    }

    pub fn get(&self, location: &I64Vec2) -> Option<&Cell> {
        self.squares.get(location)
    }

    pub fn feed_organism(
        &mut self,
        organism: &Arc<RwLock<Organism>>,
        location: I64Vec2,
    ) -> Result<(), anyhow::Error> {
        let mut consumed = 0;

        for adjustment in AroundSquare::new() {
            let looking_at = location + adjustment;

            match self.squares.get(&looking_at) {
                Some(Cell::Food) => {
                    self.squares.remove(&looking_at);
                    consumed += 1;
                }
                Some(_) => {}
                None => {}
            };
        }

        let mut org_lock = organism.write().unwrap();
        org_lock.feed(consumed);

        Ok(())
    }
    pub fn kill_around(
        &mut self,
        killer: &Arc<RwLock<Organism>>,
        killer_organ_location: I64Vec2,
    ) -> (Vec<Arc<RwLock<Organism>>>, Vec<anyhow::Error>) {
        let mut kill_list = Vec::new();

        for adjustment in AroundSquare::new() {
            let looking_at = killer_organ_location + adjustment;
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

        let mut actually_killed = Vec::new();
        let mut errors = Vec::new();

        for organism in kill_list.iter() {
            match self.kill_organism(organism) {
                Ok(()) => actually_killed.push(Arc::clone(organism)),
                Err(e) => {
                    //most likely the organism is already dead, do nothing
                    errors.push(e)
                }
            }
        }

        (actually_killed, errors)
    }

    pub fn kill_organism(
        &mut self,
        dead_organism: &Arc<RwLock<Organism>>,
    ) -> Result<(), anyhow::Error> {
        let locations_to_remove: Vec<I64Vec2> =
            { dead_organism.read().unwrap().occupied_locations().collect() };

        for location in locations_to_remove.clone() {
            if self.insert(location, Cell::Food).is_none() {
                //this can happen if an organism that is being killed just reproduced
                return Err(anyhow!(
                    "Somehow, the organism is not in locations to remove!\nOrganism: {:?}\nlocations_to_remove: {:?}\nlocation: {}",
                    dead_organism.read().unwrap(),
                    locations_to_remove,
                    location
                ));
            };
        }

        Ok(())
    }

    pub fn move_organism_with_eyes(
        &mut self,
        organism: &Arc<RwLock<Organism>>,
        eyes: Vec<(I64Vec2, Direction)>,
    ) -> Result<I64Vec2, anyhow::Error> {
        let head_towards = {
            let org_lock = organism.read().unwrap();
            let mut best_move = None;
            //eyes can see five blocks ahead
            for (eye_location, direction) in eyes {
                for block_number in 1..=5 {
                    let look_to = direction.delta() * block_number;
                    match (&best_move, self.get(&(eye_location + look_to))) {
                        (
                            None | Some(BestMoveReason::Wall(_, _)),
                            Some(Cell::Organism(_, organ)),
                        ) => {
                            let o = organ.read().unwrap();
                            if o.r#type == OrganType::Killer {
                                best_move =
                                    Some(BestMoveReason::Danger(block_number as u64, direction));
                            }
                            break;
                        }
                        (None | Some(BestMoveReason::Wall(_, _)), Some(Cell::Food)) => {
                            best_move = Some(BestMoveReason::Food(block_number as u64, direction));
                            break;
                        }
                        (None, Some(Cell::Wall)) => {
                            best_move = Some(BestMoveReason::Wall(block_number as u64, direction));
                            break;
                        }
                        (Some(BestMoveReason::Wall(distance, _)), Some(Cell::Wall)) => {
                            if (block_number as u64) < *distance {
                                best_move =
                                    Some(BestMoveReason::Wall(block_number as u64, direction))
                            }
                            break;
                        }
                        (Some(_), Some(Cell::Wall)) => {
                            break;
                        }

                        (
                            Some(
                                BestMoveReason::Food(distance, _)
                                | BestMoveReason::Danger(distance, _),
                            ),
                            Some(Cell::Food),
                        ) => {
                            if (block_number as u64) < *distance {
                                best_move =
                                    Some(BestMoveReason::Food(block_number as u64, direction));
                            }
                            break;
                        }
                        (
                            Some(
                                BestMoveReason::Food(distance, _)
                                | BestMoveReason::Danger(distance, _),
                            ),
                            Some(Cell::Organism(_, organ)),
                        ) => {
                            let o = organ.read().unwrap();
                            if o.r#type == OrganType::Killer && (block_number as u64) < *distance {
                                best_move =
                                    Some(BestMoveReason::Food(block_number as u64, direction));
                            }
                            break;
                        }
                        (
                            Some(
                                BestMoveReason::Danger(d, _)
                                | BestMoveReason::Food(d, _)
                                | BestMoveReason::Wall(d, _),
                            ),
                            None,
                        ) => {
                            if *d <= block_number as u64 {
                                break;
                            }
                        }
                        (None, None) => {}
                    }
                }
            }

            match best_move {
                Some(BestMoveReason::Danger(_, d) | BestMoveReason::Wall(_, d)) => d.to_reversed(),
                Some(BestMoveReason::Food(_, d)) => d,
                None => org_lock.facing(),
            }
        };

        self.move_organism(organism, head_towards.delta())?;

        Ok(head_towards.delta())
    }

    pub fn move_organism(
        &mut self,
        organism: &Arc<RwLock<Organism>>,
        move_by: I64Vec2,
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
            self.insert(location + move_by, Cell::organism(organism, organ));

            self.remove(location);
        }

        org_lock.move_by(move_by);
        Ok(())
    }

    pub fn produce_food_around(&mut self, location: I64Vec2) -> Result<(), anyhow::Error> {
        for adjustment in AroundSquare::new() {
            let random_spot = location + adjustment;
            if self.get(&random_spot).is_none() {
                self.insert(random_spot, Cell::Food);
                return Ok(());
            }
        }
        Err(anyhow!(
            "Could not spawn food after three randomized attempts!"
        ))
    }

    pub fn deliver_child(
        &mut self,
        parent: &Arc<RwLock<Organism>>,
        spawn_radius: u64,
    ) -> Result<Arc<RwLock<Organism>>, anyhow::Error> {
        let mut parent = parent.write().unwrap();

        let new_spawn = match parent.reproduce() {
            Ok(spawn) => spawn,
            Err(e) => {
                return Err(anyhow!("The parent failed to produce a new spawn: {}", e));
            }
        };

        let mut rng = rand::thread_rng();
        let basis = parent.location;

        let mut attempt_count = 0;
        let baby_location: I64Vec2 = loop {
            let x = rng.gen_range(-(spawn_radius as i64)..=spawn_radius as i64);
            let y = rng.gen_range(-(spawn_radius as i64)..=spawn_radius as i64);
            let new_basis = basis + I64Vec2::new(x, y);
            //there could be an instance where it's on the edge and this doesn't work, so then the insert organism code should prevent the other case.
            if let Some(wall_half) = self.settings.wall_length_half {
                if new_basis.x <= -wall_half
                    || new_basis.x >= wall_half
                    || new_basis.y <= -wall_half
                    || new_basis.y >= wall_half
                {
                    continue;
                }
            }

            let mut valid_basis = true;

            for (_abs, organ) in parent.organs() {
                match self.squares.get(&(new_basis + organ.relative_location)) {
                    None => {}
                    _ => {
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

        self.insert_organism(&new_organism)?;

        Ok(new_organism)
    }

    pub fn check(&self, location: &I64Vec2) -> Option<&Cell> {
        self.squares.get(location)
    }

    pub fn remove(&mut self, location: I64Vec2) -> Option<Cell> {
        self.squares.remove(&location)
    }
    pub fn insert(&mut self, location: I64Vec2, block: Cell) -> Option<Cell> {
        self.squares.insert(location, block)
    }

    pub fn iter(&self) -> Iter<'_, I64Vec2, Cell> {
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
            if let Some(wall_half) = self.settings.wall_length_half {
                if location.x <= -wall_half
                    || location.x >= wall_half
                    || location.y <= -wall_half
                    || location.y >= wall_half
                {
                    return Err(anyhow!(
                        "Cannot insert an organism past the wall! location in question: {}",
                        location
                    ));
                }
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
    pub fn get_organisms_touching(&self, location: I64Vec2) -> Option<Vec<I64Vec2>> {
        let mut touching_organisms = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let check = location + I64Vec2::new(i, j);

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
    type Item = (&'a I64Vec2, &'a Cell);
    type IntoIter = <&'a std::collections::HashMap<
        bevy::math::I64Vec2,
        Cell,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter()
    }
}
impl<'a> IntoIterator for &'a mut WorldMap {
    type Item = (&'a I64Vec2, &'a mut Cell);
    type IntoIter = <&'a mut std::collections::HashMap<
        bevy::math::I64Vec2,
        Cell,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter_mut()
    }
}

impl IntoIterator for WorldMap {
    type Item = (I64Vec2, Cell);
    type IntoIter = <std::collections::HashMap<
        bevy::math::I64Vec2,
        Cell,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.into_iter()
    }
}

pub enum BestMoveReason {
    Food(u64, Direction),
    Danger(u64, Direction),
    Wall(u64, Direction),
}

impl Default for AroundSquare {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AroundSquare {
    x_val: bool,
    y_val: bool,
    counter: u8,
}
impl AroundSquare {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x_val = rng.gen::<bool>();
        let y_val = rng.gen::<bool>();

        Self {
            x_val,
            y_val,
            counter: 0,
        }
    }
}

impl Iterator for AroundSquare {
    type Item = I64Vec2;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 3 {
            return None;
        }

        self.counter += 1;
        (self.x_val, self.y_val) = match (self.x_val, self.y_val) {
            (true, true) => (true, false),
            (true, false) => (false, false),
            (false, false) => (false, true),
            (false, true) => (true, true),
        };

        let (x, y) = match (self.x_val, self.y_val) {
            (true, true) => (1, 0),
            (true, false) => (0, 1),
            (false, false) => (-1, 0),
            (false, true) => (0, -1),
        };
        Some(I64Vec2::new(x, y))
    }
}
