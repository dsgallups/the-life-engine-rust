use bevy::prelude::*;

use crate::gameplay::genome::Genome;

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(Component, Clone, Debug)]
pub struct Organism {
    genome: Genome,
}

impl Organism {
    pub fn new(genome: Genome) -> Self {
        Self { genome }
    }
}
