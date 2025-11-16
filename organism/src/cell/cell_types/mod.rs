// mod brain;
// pub use brain::*;

mod data;
pub use data::*;

mod foot;
pub use foot::*;

mod launcher;
pub use launcher::*;

mod eye;
pub use eye::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((data::plugin, foot::plugin, launcher::plugin, eye::plugin));
}
