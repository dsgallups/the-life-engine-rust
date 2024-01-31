use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::startup::StartupPlugin;
mod components;
mod peripherals;
use peripherals::*;
mod world;
use world::*;
mod organism;
mod startup;

fn main() {
    println!("rewrite #3! this time, using ECS!!!");

    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(0.05))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin, StartupPlugin))
        .add_systems(Update, (move_camera, frame_update, text_fps_system))
        .add_systems(FixedUpdate, fixed_update)
        .run();
}
