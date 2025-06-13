#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

mod produce;
pub use produce::*;

mod grid;

mod coords;
pub use coords::*;

mod query;
pub use query::*;

use std::marker::PhantomData;

use bevy::{ecs::system::SystemParam, prelude::*};
use rand::seq::{IndexedRandom, SliceRandom};

use crate::gameplay::{GameSet, GameState};

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum GridSet {
    /// Prepares the environment grid for querying in the update schedule.
    ReadyGrid,
    /// Rectifies the grid based on events in Update
    RectifyGrid,
    SyncTransforms,
}

pub(super) fn plugin(app: &mut App) {
    //app.configure_sets()

    app.configure_sets(
        Update,
        (
            GridSet::ReadyGrid,
            GameSet::Update,
            GridSet::RectifyGrid,
            GridSet::SyncTransforms,
        )
            .chain(),
    );

    app.add_plugins((grid::plugin, produce::plugin, coords::plugin, query::plugin));
}

// /// Note: the assumption is that
// fn clear_previous_coords_from_grid(
//     coords: Query<&InitialFrameCoords, Changed<GlobalCoords>>,
//     mut grid: ResMut<WorldGrid>,
// ) {
//     for previous_coords in coords {
//         grid.map.remove(&previous_coords.0);
//     }
// }

// fn add_new_coords_to_grid(
//     coords: Query<(Entity, &GlobalCoords), Changed<GlobalCoords>>,
//     mut grid: ResMut<WorldGrid>,
// ) {
//     for (entity, coords) in coords {
//         if grid.map.insert(coords.0, entity).is_some() {
//             panic!("Added new coords, but something was already there!");
//         }
//     }
// }
