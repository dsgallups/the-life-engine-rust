mod grid;
mod ui;

use bevy::prelude::*;

use crate::{genome::Genome, organism::SpawnOrganism, utils::Random};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, ui::plugin));
    app.add_systems(Startup, spawn_first_organism);
}

fn spawn_first_organism(mut msgs: MessageWriter<SpawnOrganism>, mut rand: ResMut<Random>) {
    msgs.write(SpawnOrganism::new(Genome::sandbox(&mut rand.0), Vec2::ZERO));
}
