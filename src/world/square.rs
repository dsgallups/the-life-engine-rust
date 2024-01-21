use std::sync::{Arc, Mutex};

use bevy::math::I64Vec3;
use rustc_hash::FxHashMap;

use crate::Organism;

#[derive(Debug)]
pub enum Square {
    Food,
    Organism(Arc<Mutex<Organism>>),
}

pub struct Map {
    squares: FxHashMap<I64Vec3, Square>,
}
