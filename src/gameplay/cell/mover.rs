use bevy::prelude::*;

use crate::gameplay::cell::{CellType, OrganismCellType};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Mover>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Organism(OrganismCellType::Mover))]
pub struct Mover;
