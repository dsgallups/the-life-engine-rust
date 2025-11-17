use bevy::prelude::*;

use crate::{
    cell::{CellOf, Cells},
    cpu_net::Cell,
    organism::OrganismSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_inputs.in_set(OrganismSet::ProcessOutput));
}

#[derive(Component, Default)]
pub struct Foot {}

fn update_inputs(
    feet: Query<(&Foot, &Cell, &CellOf)>,
    mut organisms: Query<&mut Transform, With<Cells>>,
) {
    //let delta = time.delta_secs();
    for (_, input, cell_of) in feet {
        let Ok(mut organism_trns) = organisms.get_mut(cell_of.0) else {
            continue;
        };
        let dir_x = input.get(0);
        let dir_y = input.get(1);
        let magnitude = input.get(2);

        let magnitude = magnitude.clamp(0_f32, 1_f32);

        organism_trns.translation.x += dir_x.clamp(-1., 1.) * magnitude;
        organism_trns.translation.y += dir_y.clamp(-1., 1.) * magnitude;
    }

    //todo
}
