#![doc = r#"
This plugin will manage:
1. despawning of entities
2. synchronization of transform for all entities
3. a query parameter to search for entities
"#]

mod actions;
pub use actions::*;

mod grid;

mod coords;
pub use coords::*;

mod query;
pub use query::*;

use bevy::prelude::*;

use crate::gameplay::GameSet;

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

    app.add_plugins((grid::plugin, actions::plugin, coords::plugin, query::plugin));
}
