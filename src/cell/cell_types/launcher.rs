use bevy::prelude::*;

use crate::{cpu_net::Cell, organism::OrganismSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_inputs.in_set(OrganismSet::ProcessOutput));
}

#[derive(Component, Default)]
pub struct Launcher {}

fn update_inputs(feet: Query<(&Launcher, &Cell)>, time: Res<Time>) {
    let delta = time.delta_secs();
    for (_, input) in feet {
        let should_fire = input.get(0);
        if should_fire <= 0. {
            continue;
        }
        let dir_x = input.get(1);
        let dir_y = input.get(2);
        //will fire in the future
        // organism_trns.translation.x += dir_x.clamp(-1., 1.) * delta;
        // organism_trns.translation.x += dir_y.clamp(-1., 1.) * delta;
    }

    //todo
}
