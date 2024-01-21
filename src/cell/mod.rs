use bevy::render::color::Color;

use crate::organism;

use super::Drawable;

#[derive(Clone, Debug)]
pub enum Cell {
    Inert(InertCell),
    Organism(OrganType),
}
impl Default for Cell {
    fn default() -> Self {
        Cell::Inert(InertCell::Empty)
    }
}

impl Drawable for Cell {
    fn color(&self) -> Color {
        match self {
            Cell::Inert(inert_cell) => inert_cell.color(),
            Cell::Organism(organism_cell) => organism_cell.color(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum InertCell {
    Food,
    Wall,
    #[default]
    Empty,
}

impl Drawable for InertCell {
    fn color(&self) -> Color {
        match self {
            InertCell::Food => Color::GREEN,
            InertCell::Wall => Color::BLUE,
            InertCell::Empty => Color::BLACK,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OrganType {
    Mouth,
    Producer(Producer),
    Mover,
    Killer,
    Armor,
    Eye,
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
            OrganType::Mover => Color::BLUE,
            OrganType::Killer => Color::RED,
            OrganType::Armor => Color::PURPLE,
            OrganType::Eye => Color::SALMON,
        }
    }
}

#[derive(Clone, Debug, Default)]
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
