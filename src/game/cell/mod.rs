mod brain;
pub use brain::*;

mod data;
pub use data::*;

mod defender;
pub use defender::*;

mod launcher;
pub use launcher::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        brain::plugin,
        data::plugin,
        defender::plugin,
        launcher::plugin,
    ));
    //todo
}
