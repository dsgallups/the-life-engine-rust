#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

use bevy::{platform::collections::HashMap, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldGrid>();
    //todo
}

#[derive(Hash, Debug, Clone, Copy)]
pub struct Coords {
    x: i32,
    y: i32,
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    grid: HashMap<Coords, Entity>,
}
