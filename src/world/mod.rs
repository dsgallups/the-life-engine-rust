use bevy::{prelude::*, utils::Uuid};

use crate::organism::{Cell, Organ, Organism};

#[derive(Resource)]
pub struct LEWorld {
    width: u64,
    height: u64,
    organisms: Vec<Organism>,
}

impl Default for LEWorld {
    fn default() -> Self {
        pub use Cell::*;
        let organs = vec![
            Organ::new(Producer, (-1, -1, 1).into()),
            Organ::new(Mouth, (0, 0, 1).into()),
            Organ::new(Producer, (1, 1, 1).into()),
        ];

        let first_organism = Organism::new(organs, (0, 0, 1).into());
        LEWorld {
            width: 20,
            height: 20,
            organisms: vec![first_organism],
        }
    }
}

impl LEWorld {
    pub fn new(width: u64, height: u64) -> LEWorld {
        LEWorld {
            width,
            height,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    pub fn width(&self) -> u64 {
        self.width
    }
    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn organisms(&self) -> &[Organism] {
        &self.organisms
    }
}

#[derive(Component)]
pub enum ItemType {
    Organism(Uuid),
}
