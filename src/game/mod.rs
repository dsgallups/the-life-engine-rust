mod grid;
mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, ui::plugin));
}

// fn spawn_first_organism(mut msgs: MessageWriter<SpawnOrganism>) {
//     let mut rng = StdRng::seed_from_u64(18912);
//     //msgs.write(SpawnOrganism::new(Genome::sandbox(&mut rng), Vec2::ZERO));
// }
