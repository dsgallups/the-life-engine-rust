use crate::Vec2d;
use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub enum Cell {
    Mouth,
    #[default]
    Producer,
}

#[derive(Clone, Debug, Default)]
pub struct Organ {
    cell: Cell,
    relative_location: Vec2d,
}

impl Organ {
    pub fn new(cell: Cell, relative_location: Vec2d) -> Organ {
        Organ {
            cell,
            relative_location,
        }
    }
}

#[derive(Default)]
pub struct Organism {
    organs: Vec<Organ>,
    location: Vec2d,
}

impl Organism {
    pub fn new(organs: Vec<Organ>, location: Vec2d) -> Organism {
        Organism { organs, location }
    }
}
