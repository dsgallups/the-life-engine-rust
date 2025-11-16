use bevy::prelude::*;

use crate::{cpu_net::Cell, organism::OrganismSet};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_outputs.in_set(OrganismSet::ProcessInput));
    app.add_systems(Update, update_inputs.in_set(OrganismSet::ProcessOutput));
    //todo
}

#[derive(Component, Default)]
pub struct DataCell {
    data: [f32; 4],
}

fn update_outputs(data_cells: Query<(&DataCell, &Cell)>) {
    for (cell, outputs) in data_cells {
        outputs.set(0, cell.data[0]);
        outputs.set(1, cell.data[0]);
        outputs.set(2, cell.data[0]);
        outputs.set(3, cell.data[0]);
    }
}

fn update_inputs(data_cells: Query<(&mut DataCell, &Cell)>) {
    for (mut cell, input) in data_cells {
        cell.data[0] = input.get(0);
        cell.data[1] = input.get(1);
        cell.data[2] = input.get(2);
        cell.data[3] = input.get(3);
    }

    //todo
}
