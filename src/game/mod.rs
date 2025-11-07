mod grid;
mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, ui::plugin));
}
