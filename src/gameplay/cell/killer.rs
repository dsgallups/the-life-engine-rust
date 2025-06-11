use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Killer>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Killer;
