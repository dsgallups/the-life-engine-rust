mod brain;
pub use brain::*;

mod data;
pub use data::*;

mod collagen;
pub use collagen::*;

mod launcher;
pub use launcher::*;

mod eye;
pub use eye::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        brain::plugin,
        data::plugin,
        collagen::plugin,
        launcher::plugin,
    ));
    //todo
}
