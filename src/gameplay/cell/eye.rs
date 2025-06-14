use bevy::prelude::*;

use crate::gameplay::cell::{CellType, OrganismCellType};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Eye>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Organism(OrganismCellType::Eye))]
pub struct Eye;
