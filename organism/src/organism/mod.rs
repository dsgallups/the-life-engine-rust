mod spawn;
pub use spawn::*;

use crate::{
    genome::Genome, //old_genome::Genome,
};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganismSet {
    ProcessInput,
    Brain,
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
        (
            OrganismSet::ProcessInput,
            OrganismSet::Brain,
            OrganismSet::ProcessOutput,
        )
            .chain(),
    );

    app.add_plugins(spawn::plugin);
}
