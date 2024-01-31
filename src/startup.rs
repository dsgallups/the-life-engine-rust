use bevy::{math::I64Vec2, prelude::*};

use crate::{
    components::{FpsText, MousePosBox, Position},
    organism::Organism,
};

pub struct StartupPlugin;

#[derive(Resource)]
pub struct WorldSettings {
    pub producer_probability: u8,
    //every nth tick of an organism being alive, decrease its food consumed by 1
    pub hunger_tick: u64,
    pub spawn_radius: u64,
    pub max_organisms: Option<usize>,
    pub wall_length_half: Option<i64>,
}

impl Default for WorldSettings {
    fn default() -> Self {
        WorldSettings {
            hunger_tick: 30,
            producer_probability: 5,
            spawn_radius: 15,
            max_organisms: None,
            wall_length_half: None,
        }
    }
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings::default())
            .add_systems(Startup, (spawn_camera, spawn_text, spawn_first_organism));
    }
}

fn spawn_camera(mut commands: Commands) {
    let transform =
        Transform::from_scale(Vec3::new(0.04, 0.04, 1.)).with_translation(Vec3::new(0., -2., 100.));

    let camera = Camera2dBundle {
        transform,
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_first_organism(mut commands: Commands) {
    let parent = commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            Position(I64Vec2::new(0, 0)),
            Organism,
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform:
                }
            ))
        })

    //todo
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/fira.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        color: Color::WHITE,
        font_size: 32.0,
    };

    commands.spawn((
        TextBundle::from_section("(0, 0)", text_style).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        MousePosBox,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font,
                    font_size: 50.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 50.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        FpsText,
    ));
}
