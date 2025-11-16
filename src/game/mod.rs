mod grid;
mod ui;

use bevy::prelude::*;
use organism::{SpawnOrganism, genome::Genome};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, ui::plugin));
    app.add_systems(Startup, spawn_first_organism);
}

fn spawn_first_organism(mut msgs: MessageWriter<SpawnOrganism>) {
    //let mut rng = StdRng::seed_from_u64(18912);
    msgs.write(SpawnOrganism::new(Genome::sandbox(), Vec2::ZERO));
}
