use bevy::prelude::*;

use crate::gameplay::cell::CellType;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Killer>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Killer)]
pub struct Killer;
