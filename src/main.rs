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
use rand::prelude::*;
fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldEnvironment>()
        .add_startup_system(spawn_camera)
        .add_startup_system(setup)
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut env: ResMut<WorldEnvironment<'static>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (num_rows, num_cols) = (env.grid_map.num_rows, env.grid_map.num_cols);
    let window = window_query.get_single().unwrap();

    let window_minimum_size = if window.height() < window.width() {
        window.height()
    } else {
        window.width()
    };

    let grid_min_size = if num_rows < num_cols {
        num_rows as f32
    } else {
        num_cols as f32
    };

    let cell_size = window_minimum_size / grid_min_size;

    let mut x = 0;
    loop {
        let mut y = 0;

        loop {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()),
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    cell_size * x as f32,
                    cell_size * y as f32,
                    0.0,
                )),
                ..default()
            });

            if y > num_rows {
                break;
            }
            y += 1;
        }

        if x > num_cols {
            break;
        }
        x += 1;
    }
}

pub fn spawn_first_organism(
    mut commands: Commands,
    mut env: ResMut<WorldEnvironment<'static>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
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

    let mut first_organism = Organism::new_with_anatomy(first_organism_anatomy);
    let window = window_query.get_single().unwrap();
    first_organism.abs_x = (window.width() / 2.0) as u64;
    first_organism.abs_y = (window.height() / 2.0) as u64;
}
