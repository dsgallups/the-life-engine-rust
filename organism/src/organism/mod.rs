mod spawn;
pub use spawn::*;

use crate::{
    genome::Genome, //old_genome::Genome,
};
use bevy::prelude::*;

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
    app.add_plugins(spawn::plugin);
}
