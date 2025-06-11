use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Armor>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Armor;
