use bevy::prelude::*;

use crate::organism::{Cell, Organ, Organism};

#[derive(Resource)]
pub struct LEWorld {
    width: usize,
    height: usize,
    organisms: Vec<Organism>,
}

impl Default for LEWorld {
    fn default() -> Self {
        pub use Cell::*;
        let organs = vec![
            Organ::new(Producer, (-1, -1).into()),
            Organ::new(Mouth, (0, 0).into()),
            Organ::new(Producer, (1, 1).into()),
        ];

        let first_organism = Organism::new(organs, (0, 0).into());
        LEWorld {
            width: 0,
            height: 0,
            organisms: vec![first_organism],
        }
    }
}

impl LEWorld {
    pub fn new(width: usize, height: usize) -> LEWorld {
        LEWorld {
            width,
            height,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }
}
