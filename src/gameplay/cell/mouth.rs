use bevy::prelude::*;

use crate::gameplay::cell::{CellType, OrganismCellType};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Mouth>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Organism(OrganismCellType::Mouth))]
pub struct Mouth;
