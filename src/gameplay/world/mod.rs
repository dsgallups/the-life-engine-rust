#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

use bevy::{platform::collections::HashMap, prelude::*};

pub(super) fn plugin(app: &mut App) {
    //app.configure_sets()
    app.init_resource::<WorldGrid>();
    //todo
}

#[derive(Component, Deref, DerefMut)]
pub struct GlobalCoords(pub IVec2);

#[derive(Resource, Default)]
pub struct WorldGrid {
    grid: HashMap<IVec2, Entity>,
}

pub struct WorldSpatialQuery {
    //todo
}
