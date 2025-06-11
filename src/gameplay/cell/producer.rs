use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Producer>();
    //todo
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Producer {
    counter: u16,
}
