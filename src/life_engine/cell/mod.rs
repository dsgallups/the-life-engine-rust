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

#[derive(Clone, Debug, Default)]
pub enum OrganismCell {
    Mouth,
    #[default]
    Producer,
}

impl Drawable for OrganismCell {
    fn color(&self) -> Color {
        match self {
            OrganismCell::Producer => Color::GREEN,
            OrganismCell::Mouth => Color::RED,
        }
    }
}
