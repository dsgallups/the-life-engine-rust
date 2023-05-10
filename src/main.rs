#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod engine;
pub mod environment;
pub mod organism;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use organism::Organism;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Engine)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_environment)
        .add_system(use_engine)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
pub fn spawn_environment(mut commands: Commands) {
    //create a 2d pixel grid
    let engine = engine::Engine::default();
}

pub fn use_engine(mut commands: Commands) {}

pub struct Engine;

impl Plugin for Engine {
    fn build(&self, app: &mut App) {}
}
