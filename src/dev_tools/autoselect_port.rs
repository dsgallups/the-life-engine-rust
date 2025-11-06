#![allow(unused)]
use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::once_after_delay};

use crate::io::MidiConfigureSystem;

pub(super) fn plugin(app: &mut App) {
    // app.add_systems(
    //     Update,
    //     autoselect_port.run_if(once_after_delay(Duration::from_secs(2))),
    // );
}

fn autoselect_port(mut commands: Commands, sys: Res<MidiConfigureSystem>) {
    commands.run_system_with(sys.0, 1);
}
