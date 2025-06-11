#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

use bevy::{platform::collections::HashMap, prelude::*};

enum WorldSystems {
    /// the world runs and sends events to be processed by the world
    RunTick,
    /// things to do
    SpawnNewOrganisms,
    /// cleanup organisms that have died
    CleanupOrganisms,
}

pub(super) fn plugin(app: &mut App) {
    //app.configure_sets()
    app.init_resource::<WorldGrid>();
    //todo
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    grid: HashMap<IVec2, Entity>,
}
