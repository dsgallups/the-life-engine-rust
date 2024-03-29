use std::sync::{Arc, RwLock};

use bevy::render::color::Color;
use rand::Rng;

use crate::{Direction, Organ, Organism};

pub trait Drawable {
    fn color(&self) -> Color;
}

#[derive(Clone, Debug)]
pub enum Cell {
    Food,
    Wall,
    Organism(Arc<RwLock<Organism>>, Arc<RwLock<Organ>>),
}
impl Cell {
    pub fn organism(organism: &Arc<RwLock<Organism>>, organ: &Arc<RwLock<Organ>>) -> Self {
        Self::Organism(Arc::clone(organism), Arc::clone(organ))
    }
}

impl Drawable for Cell {
    fn color(&self) -> Color {
        match self {
            Cell::Food => Color::BLUE,
            Cell::Wall => Color::DARK_GRAY,
            Cell::Organism(_, organism_cell) => organism_cell.read().unwrap().color(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrganType {
    Mouth,
    Producer(Producer),
    Mover,
    Killer,
    Armor,
    Eye(Direction),
}

impl OrganType {
    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=5) {
            0 => Self::Mouth,
            1 => Self::Producer(Producer::default()),
            2 => Self::Mover,
            3 => Self::Killer,
            4 => Self::Armor,
            5 => Self::Eye(Direction::rand()),
            _ => panic!(),
        }
    }
}

impl Default for OrganType {
    fn default() -> Self {
        OrganType::Producer(Producer::default())
    }
}

impl Drawable for OrganType {
    fn color(&self) -> Color {
        match self {
            OrganType::Producer(_) => Color::GREEN,
            OrganType::Mouth => Color::ORANGE,
            OrganType::Mover => Color::AQUAMARINE,
            OrganType::Killer => Color::RED,
            OrganType::Armor => Color::PURPLE,
            OrganType::Eye(_) => Color::SALMON,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Producer {
    pub food_produced: u8,
    pub counter: u8,
}

impl Producer {
    pub fn new() -> Producer {
        Producer {
            food_produced: 0,
            counter: 0,
        }
    }
}
