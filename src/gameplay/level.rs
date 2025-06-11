use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_first_organism);
    //todo
}

fn spawn_first_organism(mut commands: Commands) {
    //todo
}
