use crate::OrganismCell;
use bevy::math::I64Vec3;
use std::fmt::Debug;
use uuid::Uuid;

use super::Producer;

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
    pub location: I64Vec3,
    has_eye: bool,
    food_collected: u64,
}

impl Organism {
    pub fn new(organs: Vec<Organ>, location: I64Vec3) -> Organism {
        Organism {
            id: Uuid::new_v4(),
            organs,
            has_eye: false,
            location,
            food_collected: 0,
        }
    }

    pub fn new_simple(location: I64Vec3) -> Organism {
        let organs = vec![
            Organ::new(OrganismCell::Producer(Producer::new()), (-1, 1, 1).into()),
            Organ::new(OrganismCell::Mouth, (0, 0, 1).into()),
            Organ::new(OrganismCell::Producer(Producer::new()), (1, -1, 1).into()),
        ];

        Organism::new(organs, location)
    }

    pub fn occupied_locations(&self) -> impl Iterator<Item = I64Vec3> + '_ {
        return self
            .organs
            .iter()
            .map(|organ| self.location + organ.relative_location);
    }
}
