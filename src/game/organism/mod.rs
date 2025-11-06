mod genome;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(genome::plugin);
    //todo
}

#[derive(Component, Reflect)]
pub struct Organism(Vec<Entity>);
