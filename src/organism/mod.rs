pub mod anatomy;
use crate::environment::Environment;

pub struct Organism {
    c: u64,
    r: u64,
    env: dyn Environment,
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
    parent: Option<Box<Organism>>,
}

impl Default for Organism {
    fn default() -> Self {
        Organism {
            c: 0,
            r: 0,
            env: Environment::default(),
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
        }
    }
}

impl Organism {
    pub fn inherit(parent: &Organism) -> Self {
        Organism::default()
    }
}
