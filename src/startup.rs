use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle},
    text::{BreakLineOn, Text2dBounds},
    window::PrimaryWindow,
};
use rand::random;

use crate::{
    environment::WorldEnvironment,
    organism::{
        anatomy::Anatomy,
        cell::{Cell, CellType},
        Organism,
    },
    MousePosBox, WORLD_HEIGHT, WORLD_WIDTH,
};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, setup, spawn_first_organism));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let transform = Transform::from_scale(Vec3::new(0.1, 0.1, 5.)).with_translation(Vec3::new(
        (WORLD_WIDTH as f32) / 2.0,
        (WORLD_HEIGHT as f32) / 2.0,
        0.,
    ));

    let camera = Camera2dBundle {
        transform,
        //transform: Transform::from_xyz(0., 0., 0.),
        //is_active: true,
        ..default()
    };

    commands.spawn(camera);
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut env: ResMut<WorldEnvironment<'static>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    env.grid_map.num_rows = WORLD_HEIGHT;
    env.grid_map.num_cols = WORLD_WIDTH;

    let mut x: f32 = 0.0;
    while x < WORLD_WIDTH as f32 {
        let mut y: f32 = 0.0;
        while y < WORLD_HEIGHT as f32 {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(random::<f32>(), random::<f32>(), random::<f32>()),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            });
            y += 1.;
        }
        x += 1.;
    }

    //center
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(0.25).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
        ..default()
    });

    let font = asset_server.load("fonts/fira.ttf");

    let text_style = TextStyle {
        font,
        color: Color::RED,
        font_size: 32.0,
    };
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new("(000.0, 000.0)", text_style)],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::new(0.2, 0.2, 1.)),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
        MousePosBox,
    ));
}

pub fn spawn_first_organism(
    commands: Commands,
    env: ResMut<WorldEnvironment<'static>>,
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
