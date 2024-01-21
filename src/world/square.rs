use std::sync::{Arc, Mutex};

use crate::Organism;

#[derive(Debug)]
pub enum Square {
    Food,
    Organism(Arc<Mutex<Organism>>),
}
