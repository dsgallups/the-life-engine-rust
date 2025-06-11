#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

use bevy::{platform::collections::HashMap, prelude::*};

use crate::gameplay::EnvironmentSet;

pub(super) fn plugin(app: &mut App) {
    //app.configure_sets()
    app.init_resource::<WorldGrid>();

    app.add_systems(
        Update,
        (
            update_previous_coords.in_set(EnvironmentSet::SetPrevCoords),
            (clear_previous_coords_from_grid, add_new_coords_to_grid)
                .in_set(EnvironmentSet::FirstGridUpdate),
            (clear_previous_coords_from_grid, add_new_coords_to_grid)
                .in_set(EnvironmentSet::SecondGridUpdate),
            sync_transform_with_coords.in_set(EnvironmentSet::SyncTransforms),
        ),
    );
    //todo
}

#[derive(Component, Deref, DerefMut)]
#[require(Transform)]
#[require(PrevGlobalCoords)]
pub struct GlobalCoords(pub IVec2);

impl GlobalCoords {
    fn as_translation(&self) -> Vec3 {
        Vec3::new(self.0.x as f32, self.0.y as f32, 0.)
    }
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    map: HashMap<IVec2, Entity>,
}

#[derive(Component, Deref, DerefMut, Default)]
struct PrevGlobalCoords(IVec2);

fn update_previous_coords(mut coords: Query<(&GlobalCoords, &mut PrevGlobalCoords)>) {
    for (cur, mut prev) in &mut coords {
        prev.0 = cur.0

        //todo
    }
}

/// Note: the assumption is that
fn clear_previous_coords_from_grid(
    coords: Query<&PrevGlobalCoords, Changed<GlobalCoords>>,
    mut grid: ResMut<WorldGrid>,
) {
    for previous_coords in coords {
        grid.map.remove(&previous_coords.0);
    }
}

fn add_new_coords_to_grid(
    coords: Query<(Entity, &GlobalCoords), Changed<GlobalCoords>>,
    mut grid: ResMut<WorldGrid>,
) {
    for (entity, coords) in coords {
        if grid.map.insert(coords.0, entity).is_some() {
            panic!("Added new coords, but something was already there!");
        }
    }
}

fn sync_transform_with_coords(
    mut coords: Query<(&mut Transform, &GlobalCoords), Changed<GlobalCoords>>,
) {
    for (mut transform, coords) in &mut coords {
        *transform = transform.with_translation(coords.as_translation());
        //todo
    }
}

pub struct WorldSpatialQuery {
    //todo
}
