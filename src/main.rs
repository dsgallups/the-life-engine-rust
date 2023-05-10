#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod engine;
pub mod environment;
pub mod organism;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use environment::WorldEnvironment;
use organism::anatomy::Anatomy;
use organism::cell::{Cell, CellType};
use organism::Organism;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldEnvironment>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_first_organism)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_first_organism(mut commands: Commands, mut env: ResMut<WorldEnvironment<'static>>) {
    //We spawn a producer that is green yellow green
    let first_organism_anatomy = Anatomy::new(vec![
        Cell {
            cell_type: CellType::Producer,
            local_x: -1,
            local_y: -1,
        },
        Cell {
            cell_type: CellType::Mouth,
            local_x: 0,
            local_y: 0,
        },
        Cell {
            cell_type: CellType::Producer,
            local_x: 1,
            local_y: 1,
        },
    ]);

    let mut first_organism = Organism::default();
}
