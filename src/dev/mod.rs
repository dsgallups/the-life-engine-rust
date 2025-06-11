//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::screens::Screen;

mod debug;
mod inspector;
mod skip;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((inspector::gadget, skip::plugin, debug::plugin));
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
}
