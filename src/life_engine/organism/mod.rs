use crate::life_engine::OrganismCell;
use bevy::math::{I64Vec3, U64Vec3};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Organ {
    pub id: Uuid,
    pub cell: OrganismCell,
    pub relative_location: I64Vec3,
}

impl Organ {
    pub fn new(cell: OrganismCell, relative_location: I64Vec3) -> Organ {
        Organ {
            id: Uuid::new_v4(),
            cell,
            relative_location,
        }
    }
}

#[derive(Default)]
pub struct Organism {
    pub id: Uuid,
    pub organs: Vec<Organ>,
    pub location: U64Vec3,
    has_eye: bool,
    food_collected: u64,
}

impl Organism {
    pub fn new(organs: Vec<Organ>, location: U64Vec3) -> Organism {
        Organism {
            id: Uuid::new_v4(),
            organs,
            has_eye: false,
            location,
            food_collected: 0,
        }
    }
}
