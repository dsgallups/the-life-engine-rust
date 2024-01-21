use std::{
    collections::hash_map::{Entry, Iter},
    sync::{Arc, Mutex},
};

use bevy::math::I64Vec3;
use rustc_hash::FxHashMap;

use crate::Organism;

#[derive(Debug)]
pub enum Square {
    Food,
    Organism(Arc<Mutex<Organism>>),
}

pub struct WorldMap {
    squares: FxHashMap<I64Vec3, Square>,
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

    pub fn get(&self, location: &I64Vec3) -> Option<&Square> {
        self.squares.get(location)
    }
    pub fn insert(&mut self, location: I64Vec3, square: Square) -> Option<Square> {
        self.squares.insert(location, square)
    }
    pub fn remove(&mut self, location: &I64Vec3) -> Option<Square> {
        self.squares.remove(location)
    }
    pub fn iter(&self) -> Iter<'_, I64Vec3, Square> {
        self.squares.iter()
    }
    pub fn entry(&mut self, location: I64Vec3) -> Entry<'_, I64Vec3, Square> {
        self.squares.entry(location)
    }
}

impl<'a> IntoIterator for &'a WorldMap {
    type Item = (&'a I64Vec3, &'a Square);
    type IntoIter = <&'a std::collections::HashMap<
        bevy::math::I64Vec3,
        Square,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter()
    }
}
impl<'a> IntoIterator for &'a mut WorldMap {
    type Item = (&'a I64Vec3, &'a mut Square);
    type IntoIter = <&'a mut std::collections::HashMap<
        bevy::math::I64Vec3,
        Square,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.iter_mut()
    }
}

impl IntoIterator for WorldMap {
    type Item = (I64Vec3, Square);
    type IntoIter = <std::collections::HashMap<
        bevy::math::I64Vec3,
        Square,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    > as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.squares.into_iter()
    }
}
