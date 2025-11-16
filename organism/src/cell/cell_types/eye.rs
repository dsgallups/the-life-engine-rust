use bevy::prelude::*;

use crate::{OrganismSet, cpu_net::Cell};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_outputs.in_set(OrganismSet::ProcessInput));
}

#[derive(Component, Default)]
pub struct Eye {}

fn update_outputs(eyes: Query<(&Eye, &Cell)>) {

    //todo
}
