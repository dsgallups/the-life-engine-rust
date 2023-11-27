use bevy::render::color::Color;

use super::Drawable;

#[derive(Clone, Debug)]
pub enum Cell {
    Inert(InertCell),
    Organism(OrganismCell),
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
pub enum OrganismCell {
    Mouth,
    Producer(u8),
}

impl Default for OrganismCell {
    fn default() -> Self {
        OrganismCell::Producer(0)
    }
}

impl Drawable for OrganismCell {
    fn color(&self) -> Color {
        match self {
            OrganismCell::Producer(_) => Color::GREEN,
            OrganismCell::Mouth => Color::RED,
        }
    }
}
