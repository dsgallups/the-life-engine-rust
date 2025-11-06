mod genome;

use bevy::prelude::*;

use crate::game::organism::genome::{Genome, SpawnOrganism};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(genome::plugin);

    app.add_systems(Startup, spawn_first_organism);
}

#[derive(Component, Reflect)]
#[relationship_target(relationship = CellOf)]
pub struct Cells(Vec<Entity>);

#[derive(Component, Reflect)]
#[relationship(relationship_target = Cells)]
pub struct CellOf(pub Entity);

fn spawn_first_organism(mut msgs: MessageWriter<SpawnOrganism>) {
    msgs.write(SpawnOrganism::new(Genome::sandbox(), Vec2::ZERO));
}
