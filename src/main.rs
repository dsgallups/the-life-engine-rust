#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod environment;
pub mod organism;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use organism::Organism;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_environment)
        .add_startup_system(spawn_organism)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
