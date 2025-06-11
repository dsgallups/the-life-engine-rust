use bevy::prelude::*;

use crate::gameplay::cell::CellType;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Producer>();
    //todo
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Producer)]
pub struct Producer {
    counter: u16,
}
