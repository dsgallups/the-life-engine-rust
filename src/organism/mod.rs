pub mod anatomy;
pub mod cell;
pub mod direction;
pub mod perception;
use crate::environment::*;

use anatomy::Anatomy;
use direction::Direction;
use perception::Brain;

pub struct Organism {
    x: u64,
    y: u64,
    //env: Box<dyn Environment>,
    lifetime: u64,
    food_collected: u64,
    living: bool,
    anatomy: Anatomy,
    direction: Direction,
    rotation: Direction,
    can_rotate: bool,
    move_count: u64,
    move_range: u64,
    ignore_brain_for: u64,
    mutability: u16,
    damage: u16,
    brain: Brain,
    //this will be a problem
    parent: Option<Box<Organism>>,
}

impl Default for Organism {
    fn default() -> Self {
        Organism {
            x: 0,
            y: 0,
            //env: Box::new(WorldEnvironment::default()),
            lifetime: 0,
            food_collected: 0,
            living: true,
            anatomy: Anatomy::default(),
            direction: Direction::default(),
            rotation: Direction::default(),
            can_rotate: true,
            move_count: 0,
            move_range: 0,
            ignore_brain_for: 0,
            mutability: 0,
            damage: 0,
            brain: Brain::default(),
            parent: None,
        }
    }
}

impl Organism {
    pub fn inherit(parent: &Organism) -> Self {
        Organism::default()
    }
    pub fn new(anatomy: Anatomy, direction: Direction, rotation: Direction) -> Self {
        Organism {
            x: 0,
            y: 0,
            //env: Box::new(WorldEnvironment::default()),
            lifetime: 0,
            food_collected: 0,
            living: true,
            anatomy,
            direction,
            rotation,
            can_rotate: true,
            move_count: 0,
            move_range: 0,
            ignore_brain_for: 0,
            mutability: 0,
            damage: 0,
            brain: Brain::default(),
            parent: None,
        }
    }
}
