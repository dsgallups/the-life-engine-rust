pub mod anatomy;
pub mod cell;
pub mod direction;
pub mod perception;
use crate::environment::*;

use anatomy::Anatomy;
use direction::Direction;
use perception::Brain;

pub struct Organism<'a> {
    pub abs_x: u64,
    pub abs_y: u64,
    //env: Box<dyn Environment>,
    tick_born: u64,
    living: bool,
    anatomy: Anatomy,
    ignore_brain_for: u64,
    mutability: u16,
    damage: u16,
    brain: Brain,
    //this will be a problem
    parent: Option<&'a Organism<'a>>,
}

impl Default for Organism<'_> {
    fn default() -> Self {
        Organism {
            abs_x: 0,
            abs_y: 0,
            //env: Box::new(WorldEnvironment::default()),
            tick_born: 0,
            living: true,
            anatomy: Anatomy::default(),
            ignore_brain_for: 0,
            mutability: 0,
            damage: 0,
            brain: Brain::default(),
            parent: None,
        }
    }
}

impl Organism<'_> {
    pub fn inherit(parent: &Organism) -> Self {
        Organism::default()
    }
    pub fn new_with_anatomy(anatomy: Anatomy) -> Self {
        Organism {
            abs_x: 0,
            abs_y: 0,
            //env: Box::new(WorldEnvironment::default()),
            tick_born: 0,
            living: true,
            anatomy,
            ignore_brain_for: 0,
            mutability: 0,
            damage: 0,
            brain: Brain::default(),
            parent: None,
        }
    }
}
