use bevy::prelude::*;

use crate::gameplay::cell::{CellType, OrganismCellType};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Armor>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Organism(OrganismCellType::Armor))]
pub struct Armor;
