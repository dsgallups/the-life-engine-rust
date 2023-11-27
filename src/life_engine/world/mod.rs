use bevy::{prelude::*, utils::Uuid};

use crate::life_engine::organism::{Organ, Organism};

#[derive(Resource)]
pub struct LEWorld {
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    organisms: Vec<Organism>,
}

impl Default for LEWorld {
    fn default() -> Self {
        let default_width = 20;
        let default_height = 20;
        let default_map = vec![vec![Cell::default(); default_width]; default_height];

        pub use OrganismCell::*;

        let organs = vec![
            Organ::new(Producer, (-1, 1, 1).into()),
            Organ::new(Mouth, (0, 0, 1).into()),
            Organ::new(Producer, (1, -1, 1).into()),
        ];

        let first_organism = Organism::new(organs, (0, 0, 1).into());
        LEWorld {
            map: default_map,
            width: default_width,
            height: default_height,
            organisms: vec![first_organism],
        }
    }
}

impl LEWorld {
    pub fn new(width: usize, height: usize) -> LEWorld {
        let map = vec![vec![Cell::default(); width]; height];
        LEWorld {
            map,
            width,
            height,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: Organism) {
        self.organisms.push(organism);
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
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

pub trait Drawable {
    fn color(&self) -> Color;
}

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
