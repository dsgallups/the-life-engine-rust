#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

use bevy::{platform::collections::HashMap, prelude::*};

use crate::gameplay::GameSet;

pub(super) fn plugin(app: &mut App) {
    //app.configure_sets()
    app.init_resource::<WorldGrid>();

    app.add_systems(
        Update,
        sync_transform_with_coords.in_set(GameSet::SyncTransforms),
    );
    //todo
}

#[derive(Component, Deref, DerefMut)]
#[require(Transform)]
pub struct GlobalCoords(pub IVec2);

impl GlobalCoords {
    fn as_translation(&self) -> Vec3 {
        Vec3::new(self.0.x as f32, self.0.y as f32, 0.)
    }
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    grid: HashMap<IVec2, Entity>,
}

pub struct WorldSpatialQuery {
    //todo
}

fn sync_transform_with_coords(mut coords: Query<(&mut Transform, &GlobalCoords)>) {
    for (mut transform, coords) in &mut coords {
        *transform = transform.with_translation(coords.as_translation());
        //todo
    }
}
