mod cell_types;
pub use cell_types::*;

mod details;
pub use details::*;

mod genome;
pub use genome::*;

mod template;
pub use template::*;

use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct ActiveCell;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(cell_types::plugin);
    //todo
}
