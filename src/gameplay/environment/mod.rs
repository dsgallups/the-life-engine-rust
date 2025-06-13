#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

use bevy::{platform::collections::HashMap, prelude::*};

use crate::gameplay::{GameSet, GameState};

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum EnvironmentSet {
    /// PrevCoords is the location of something before the update passes start
    SetInitialCoords,
    UpdateGrid,
    SyncTransforms,
}

pub(super) fn plugin(app: &mut App) {
    //app.configure_sets()
    app.init_resource::<WorldGrid>();

    app.configure_sets(
        Update,
        (
            EnvironmentSet::SetInitialCoords,
            EnvironmentSet::UpdateGrid,
            EnvironmentSet::SyncTransforms,
        )
            .chain(),
    );

    app.add_systems(
        PreUpdate,
        set_initial_frame_coords.run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        (clear_previous_coords_from_grid, add_new_coords_to_grid)
            .chain()
            .in_set(GameSet::ReadyGrid),
    )
    .add_systems(
        Update,
        sync_transform_with_coords.in_set(GameSet::SyncTransforms),
    );
}

#[derive(Component, Deref, DerefMut)]
#[require(Transform)]
#[require(InitialFrameCoords)]
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
struct InitialFrameCoords(IVec2);

fn set_initial_frame_coords(mut coords: Query<(&GlobalCoords, &mut InitialFrameCoords)>) {
    for (cur, mut prev) in &mut coords {
        prev.0 = cur.0

        //todo
    }
}

/// Note: the assumption is that
fn clear_previous_coords_from_grid(
    coords: Query<&InitialFrameCoords, Changed<GlobalCoords>>,
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
