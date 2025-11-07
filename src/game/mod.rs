mod cell;
mod grid;
mod organism;
mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, organism::plugin, cell::plugin, ui::plugin));
}
