use bevy::prelude::*;

use crate::gameplay::cell::CellType;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Armor>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Armor)]
pub struct Armor;
