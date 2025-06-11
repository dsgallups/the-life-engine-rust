use bevy::prelude::*;

use crate::{
    gameplay::{genome::OrganismGenome, organism::Organism},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_first_organism);
    //todo
}

fn spawn_first_organism(mut commands: Commands, mut genomes: ResMut<Assets<OrganismGenome>>) {
    commands.spawn(Organism(genomes.add(OrganismGenome::first_organism())));
}
