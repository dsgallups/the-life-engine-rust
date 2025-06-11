use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Mouth>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Mouth;
