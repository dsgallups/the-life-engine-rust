use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    text::{BreakLineOn, Text2dBounds},
    window::PrimaryWindow,
};

use crate::{
    environment::WorldEnvironment,
    organism::{
        anatomy::Anatomy,
        cell::{Cell, CellType},
        Organism,
    },
    MousePosBox,
};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, setup, spawn_first_organism));
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let transform = Transform::from_scale(Vec3::new(0.03, 0.03, 5.)).with_translation(Vec3::new(
        500.0 / 2.0,
        500.0 / 2.0,
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
    env.grid_map.num_rows = 500;
    env.grid_map.num_cols = 500;

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
