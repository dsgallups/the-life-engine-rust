mod spawn;
pub use spawn::*;

use crate::{
    cpu_net::Cell,
    genome::Genome, //old_genome::Genome,
};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganismSet {
    ProcessInput,
    ProcessOutput,
}

#[derive(Component, Reflect)]
pub struct ActiveOrganism;

#[derive(Component)]
pub struct Organism {
    genome: Genome,
}
impl Organism {
    pub fn new(genome: Genome) -> Self {
        Self { genome }
    }
}

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (OrganismSet::ProcessInput, OrganismSet::ProcessOutput).chain(),
    );

    app.add_plugins(spawn::plugin);
    app.add_systems(PostUpdate, reset_cells);
}

fn reset_cells(
    cells: Query<&Cell>,
    // cell_outputs: Query<&CellOutput>,
    // cell_inputs: Query<&mut CellInput>,
) {
    for cell in cells {
        cell.reset();
    }
}
