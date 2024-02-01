use std::sync::{Arc, RwLock};

use bevy::{ecs::component::Component, render::color::Color};
use rand::Rng;

use crate::{direction::Direction, Organ, Organism};

pub trait Drawable {
    fn color(&self) -> Color;
}

#[derive(Clone, Debug, Component)]
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
