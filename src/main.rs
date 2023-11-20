#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod engine;
pub mod environment;
pub mod organism;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::text::{BreakLineOn, Text2dBounds};
use bevy::window::PrimaryWindow;
use environment::WorldEnvironment;
use organism::anatomy::Anatomy;
use organism::cell::{Cell, CellType};
use organism::Organism;
use rand::prelude::*;

const CELL_SIZE: f32 = 12.0;
fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldEnvironment>()
        .add_systems(Startup, (spawn_camera, setup, spawn_first_organism))
        .add_systems(Update, print_mouse_pos)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    /*commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 2.0),
        ..default()
    });*/

    commands.spawn(Camera2dBundle::default());
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut env: ResMut<WorldEnvironment<'static>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
    env.grid_map.num_rows = (CELL_SIZE / window.height()) as u64;
    env.grid_map.num_cols = (CELL_SIZE / window.width()) as u64;

    let mut x: f32 = 0.0;
    while x < window.width() {
        let mut y: f32 = 0.0;
        while y < window.height() {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()),
                    custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            });
            y += CELL_SIZE;
        }
        x += CELL_SIZE;
    }
    //center
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
        ..default()
    });

    let font = asset_server.load("fonts/fira.ttf");
    //create box for the mouse position
    let text_style = TextStyle {
        font,
        color: Color::WHITE,
        font_size: 22.0,
    };
    let box_size = Vec2::new(140.0, 30.0);
    let box_position = Vec2::new(window.width() - (box_size.x / 2.0), box_size.y / 2.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.20, 0.3, 0.70),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new("(000.0, 000.0)", text_style.clone())],
                        alignment: TextAlignment::Left,
                        linebreak_behavior: BreakLineOn::AnyCharacter,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: box_size,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                },
                MousePosBox,
            ));
        });
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

#[derive(Component)]
pub struct MousePosBox;

pub fn print_mouse_pos(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_pos_box: Query<&mut Text, With<MousePosBox>>,
) {
    let mut text = mouse_pos_box.get_single_mut().unwrap();
    let window = windows.get_single().unwrap();
    let pos = window.cursor_position();
    match pos {
        Some(pos) => text.sections[0].value = format!("({}, {})", pos.x, pos.y),
        None => text.sections[0].value = format!("(n/a, n/a)"),
    }
}
